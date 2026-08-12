use crate::db::llm_provider_settings as provider_repo;
use crate::db::message as msg_repo;
use crate::db::system_prompt as system_prompt_repo;
use crate::domain::conversations::ChatStreamEvent;
use crate::domain::llm::LlmChatCompletionRequest;
use crate::domain::llm::LlmChatCompletionResult;
use crate::domain::llm::LlmProvider;
use crate::domain::messages::Message;
use crate::domain::messages::MessageRole;
use crate::errors::AppError;
use crate::infra::llm_providers::create_llm_provider;
use crate::infra::screenshot::capture_screenshot as do_capture_screenshot;
use crate::infra::screenshot::ScreenshotResult;
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::SqlitePool;
use std::sync::LazyLock;
use tauri::Manager;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

static SESSION: LazyLock<Mutex<Option<CancellationToken>>> = LazyLock::new(|| Mutex::new(None));

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    provider: String,
    conversation_id: Uuid,
    user_message: String,
    capture_screenshot: bool,
    system_prompt_id: Option<Uuid>,
) -> Result<(), AppError> {
    tracing::trace!(
        provider,
        %conversation_id,
        %user_message,
        capture_screenshot,
        ?system_prompt_id,
        "send_message called"
    );

    let mut guard = SESSION.lock().await;
    if guard.is_some() {
        tracing::warn!("Already streaming, ignoring request");
        return Err(AppError::LlmProviderAlreadyRunning);
    }

    let screenshot_result: Option<ScreenshotResult> = if capture_screenshot {
        match do_capture_screenshot(app.clone()) {
            Ok(result) => Some(result),
            Err(e) => {
                tracing::error!(error = %e, "Failed to capture screenshot");
                None
            }
        }
    } else {
        None
    };

    let screenshot_path = screenshot_result.as_ref().map(|r| r.relative_path.clone());
    let screenshot_base64 = screenshot_result.map(|r| r.base64);

    let pool = app.state::<SqlitePool>();
    if let Err(e) = msg_repo::save(
        &pool,
        &Message {
            id: Uuid::new_v4(),
            conversation_id: conversation_id.clone(),
            role: MessageRole::User,
            content: user_message.clone(),
            screenshot_path: screenshot_path.clone(),
            created_at: Utc::now(),
        },
    )
    .await
    {
        tracing::error!(error = %e, "Error saving user message");
    }

    let provider_settings = provider_repo::get(&pool, &provider)
        .await?
        .ok_or_else(|| AppError::LlmProviderNotConfigured)?;
    let llm_provider = create_llm_provider(&provider_settings)?;

    let history = msg_repo::get_by_conversation(&pool, &conversation_id).await?;

    let system_prompt_text = if let Some(ref id) = system_prompt_id {
        system_prompt_repo::get_by_id(&pool, &id)
            .await?
            .map(|x| x.prompt)
    } else {
        None
    };

    let mut request = LlmChatCompletionRequest::new(history);

    if let Some(sp) = system_prompt_text {
        request = request.with_system_prompt(sp);
    }

    if let Some(b64) = screenshot_base64 {
        request = request.with_screenshot(b64);
    } else if capture_screenshot {
        tracing::error!("capture_screenshot=true but no base64 available");
    }

    let cancellation_token = CancellationToken::new();
    tokio::spawn({
        let app = app.clone();
        let token = cancellation_token.clone();
        async move {
            run_chat_completion(app, conversation_id, request, llm_provider, token).await;
        }
    });

    *guard = Some(cancellation_token);

    Ok(())
}

async fn run_chat_completion(
    app: AppHandle,
    conversation_id: Uuid,
    request: LlmChatCompletionRequest,
    llm_provider: Box<dyn LlmProvider>,
    ct: CancellationToken,
) {
    let mut assistant_response = String::new();

    let mut stream = match llm_provider.stream_chat_completion(request).await {
        Ok(stream) => stream,
        Err(err) => {
            tracing::error!(error = ?err, "Provider error");
            let _ = app.emit(
                "chat-stream",
                ChatStreamEvent::Error {
                    code: "provider_error".to_string(),
                    message: err.to_string(),
                },
            );
            finish().await;
            return;
        }
    };

    loop {
        tokio::select! {
            Some(event) = stream.next() => {
                match event {
                    LlmChatCompletionResult::Chunk {
                        content,
                        is_finish,
                        usage,
                    } => {
                        assistant_response.push_str(&content);

                        let _ = app.emit(
                            "chat-stream",
                            ChatStreamEvent::Chunk {
                                conversation_id: conversation_id.clone(),
                                content: content,
                                is_finish: is_finish,
                                usage: usage,
                                timestamp: Utc::now(),
                            },
                        );
                    }
                    LlmChatCompletionResult::Error { code, message } => {
                        tracing::error!(code = %code, message = %message, "LLM provider stream error");
                        let _ = app.emit(
                            "chat-stream",
                            ChatStreamEvent::Error {
                                code,
                                message
                            },
                        );
                    }
                }
            },
            _ = ct.cancelled() => break,
        }
    }

    let _ = app.emit(
        "chat-stream",
        ChatStreamEvent::Chunk {
            conversation_id: conversation_id.clone(),
            content: "".to_string(),
            is_finish: true,
            usage: None,
            timestamp: Utc::now(),
        },
    );

    tracing::trace!("Stream completed");

    if !assistant_response.is_empty() {
        let pool = app.state::<SqlitePool>();
        if let Err(e) = msg_repo::save(
            &pool,
            &Message {
                id: Uuid::new_v4(),
                conversation_id: conversation_id.clone(),
                role: MessageRole::Assistant,
                content: assistant_response,
                screenshot_path: None,
                created_at: Utc::now(),
            },
        )
        .await
        {
            tracing::error!(error = %e, "Error saving assistant message");
        }
    } else {
        tracing::trace!("Assistant response was empty, not saving");
    }
    finish().await
}

async fn finish() {
    *SESSION.lock().await = None;
}

#[tauri::command]
pub async fn stop_stream() -> Result<(), String> {
    tracing::trace!("stop_stream called");

    if let Some(session) = SESSION.lock().await.as_ref() {
        session.cancel();
    } else {
        tracing::warn!("LLM provider was not running but stop was requested");
    }

    Ok(())
}
