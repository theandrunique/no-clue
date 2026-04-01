use crate::db::transcript as transcript_repo;
use crate::models::{Speaker, TranscriptionResult};
use crate::stt_providers::get_stt_descriptors;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Serialize, Deserialize)]
pub struct SttSettings {
    pub api_key: String,
    pub model: String,
    pub language: String,
}

#[tauri::command]
pub async fn save_stt_settings(
    api_key: String,
    model: String,
    language: String,
) -> Result<(), String> {
    tracing::info!(
        model,
        language,
        api_key = !api_key.is_empty(),
        "save_stt_settings called"
    );
    Ok(())
}

#[tauri::command]
pub async fn get_stt_settings() -> Result<SttSettings, String> {
    tracing::info!("get_stt_settings called");
    Ok(SttSettings {
        api_key: "".to_string(),
        model: "nova-3".to_string(),
        language: "ru".to_string(),
    })
}

#[tauri::command]
pub fn get_stt_providers() -> Vec<crate::stt_providers::SttProviderDescriptor> {
    get_stt_descriptors()
}

static TRANSCRIPTION_RUNNING: AtomicBool = AtomicBool::new(false);
static CURRENT_CONVERSATION_ID: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
pub async fn update_transcription_session(conversation_id: String) -> Result<(), String> {
    tracing::debug!(conversation_id = %conversation_id, "update_transcription_session called");
    let mut current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
    *current = Some(conversation_id);
    Ok(())
}

#[tauri::command]
pub async fn start_transcription(app: AppHandle) -> Result<(), String> {
    let conversation_id = {
        let current = CURRENT_CONVERSATION_ID.lock().map_err(|e| e.to_string())?;
        current.clone().ok_or("No conversation ID set")?
    };

    tracing::info!(conversation_id = %conversation_id, "start_transcription called");

    if TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
        tracing::warn!("Transcription already running");
        return Err("Transcription already running".to_string());
    }

    TRANSCRIPTION_RUNNING.store(true, Ordering::SeqCst);
    let _ = app.emit("transcription-started", ());

    tracing::info!("Transcription started - using fake provider");

    let user_phrases = vec![
        "Can you help me with this code",
        "Let me explain what I mean",
        "That's exactly what I wanted",
        "Could you summarize this",
        "Thank you for your help",
    ];

    let system_phrases = vec![
        "System notification: Update available",
        "Email received from John",
        "Meeting starts in 5 minutes",
        "File download complete",
        "New message in Slack",
    ];

    let mut phrase_index = 0;
    let conv_id = conversation_id.clone();

    let app_clone = app.clone();
    tokio::spawn(async move {
        while TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
            let is_user = phrase_index % 2 == 0;
            let speaker = if is_user { "user" } else { "system" };
            let phrases = if is_user {
                &user_phrases
            } else {
                &system_phrases
            };
            let phrase = phrases[phrase_index % phrases.len()];
            let interim_id = uuid::Uuid::new_v4().to_string();
            let timestamp = chrono::Utc::now().timestamp();

            let interim_text = &phrase[..phrase.len() / 2];
            let interim_result = TranscriptionResult {
                id: interim_id,
                conversation_id: conv_id.clone(),
                text: interim_text.to_string(),
                is_final: false,
                speaker: speaker.to_string(),
                confidence: 0.7,
                timestamp,
            };
            let _ = app_clone.emit("transcription-result", interim_result);

            tracing::trace!(speaker, text = %interim_text, "Interim transcription");

            tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

            if !TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
                break;
            }

            let conv_id_clone = conv_id.clone();
            let speaker_clone = speaker.to_string();
            let phrase_clone = phrase.to_string();
            let final_id = uuid::Uuid::new_v4().to_string();
            let final_timestamp = chrono::Utc::now().timestamp();

            let transcript = tokio::task::spawn_blocking(move || {
                let speaker = if speaker_clone == "user" {
                    Speaker::User
                } else {
                    Speaker::System
                };
                transcript_repo::create(
                    final_id,
                    conv_id_clone,
                    speaker,
                    phrase_clone,
                    0.95,
                    final_timestamp,
                )
            })
            .await
            .ok()
            .and_then(|r| r.ok());

            let final_result = match transcript {
                Some(t) => TranscriptionResult::from(t),
                None => TranscriptionResult {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: conv_id.clone(),
                    speaker: speaker.to_string(),
                    text: phrase.to_string(),
                    is_final: true,
                    confidence: 0.95,
                    timestamp: chrono::Utc::now().timestamp(),
                },
            };
            let _ = app_clone.emit("transcription-result", final_result);

            tracing::trace!(speaker, text = %phrase, "Final transcription");

            phrase_index += 1;

            tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_transcription(app: AppHandle) -> Result<(), String> {
    tracing::info!("stop_transcription called");

    TRANSCRIPTION_RUNNING.store(false, Ordering::SeqCst);
    let _ = app.emit("transcription-stopped", ());

    tracing::info!("Transcription stopped");
    Ok(())
}
