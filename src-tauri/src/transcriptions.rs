use crate::db;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

static TRANSCRIPTION_RUNNING: AtomicBool = AtomicBool::new(false);
static TRANSCRIPTION_HANDLE: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub async fn update_transcription_session(conversation_id: String) -> Result<(), String> {
    println!(
        "[COMMAND] update_transcription_session called: conversation_id={}",
        conversation_id
    );
    // Store conversation ID in static
    // For simplicity, we'll use the conversation_id in the task
    Ok(())
}

// Transcription - emits random phrases every 10 seconds
#[tauri::command]
pub async fn start_transcription(app: AppHandle, conversation_id: String) -> Result<(), String> {
    println!(
        "[COMMAND] start_transcription called for conversation: {}",
        conversation_id
    );

    if TRANSCRIPTION_RUNNING.load(Ordering::SeqCst) {
        return Err("Transcription already running".to_string());
    }

    TRANSCRIPTION_RUNNING.store(true, Ordering::SeqCst);
    TRANSCRIPTION_HANDLE.store(true, Ordering::SeqCst);
    let _ = app.emit("transcription-started", ());

    println!("[COMMAND] Transcription started - simulating...");

    // Random phrases for transcription simulation
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

    // Spawn a task to emit transcription results every 10 seconds
    let app_clone = app.clone();
    tokio::spawn(async move {
        while TRANSCRIPTION_HANDLE.load(Ordering::SeqCst) {
            let is_user = phrase_index % 2 == 0;
            let speaker = if is_user { "user" } else { "system" };
            let phrases = if is_user {
                &user_phrases
            } else {
                &system_phrases
            };
            let phrase = phrases[phrase_index % phrases.len()];

            // First, emit interim (non-final) transcription
            let interim_text = &phrase[..phrase.len() / 2];
            let _ = app_clone.emit(
                "transcription-result",
                serde_json::json!({
                    "text": interim_text,
                    "is_final": false,
                    "speaker": speaker,
                    "confidence": 0.7
                }),
            );

            println!("[TRANSCRIPTION] {}: {} (interim)", speaker, interim_text);

            // Wait a bit, then emit final
            tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

            if !TRANSCRIPTION_HANDLE.load(Ordering::SeqCst) {
                break;
            }

            // Emit final transcription
            let _ = app_clone.emit(
                "transcription-result",
                serde_json::json!({
                    "text": phrase,
                    "is_final": true,
                    "speaker": speaker,
                    "confidence": 0.95
                }),
            );

            println!("[TRANSCRIPTION] {}: {} (final)", speaker, phrase);

            // Save final transcript to database
            let conv_id_clone = conv_id.clone();
            let speaker_clone = speaker.to_string();
            let phrase_clone = phrase.to_string();

            tokio::spawn(async move {
                match db::save_transcript(conv_id_clone, speaker_clone, phrase_clone, Some(0.95))
                    .await
                {
                    Ok(_) => println!("[DB] Saved transcript to database"),
                    Err(e) => println!("[DB] Error saving transcript: {}", e),
                }
            });

            phrase_index += 1;

            // Wait 10 seconds before next phrase
            tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_transcription(app: AppHandle) -> Result<(), String> {
    println!("[COMMAND] stop_transcription called");

    TRANSCRIPTION_RUNNING.store(false, Ordering::SeqCst);
    TRANSCRIPTION_HANDLE.store(false, Ordering::SeqCst);
    let _ = app.emit("transcription-stopped", ());

    println!("[COMMAND] Transcription stopped");
    Ok(())
}
