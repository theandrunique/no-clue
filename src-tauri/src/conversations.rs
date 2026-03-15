use crate::db::conversation as conv_repo;
use crate::db::message as msg_repo;
use crate::models::{Conversation, MessageRole};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

static STREAMING: AtomicBool = AtomicBool::new(false);

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    println!("[COMMAND] create_conversation called");

    let title = "New conversation".to_string();
    let id = tokio::task::spawn_blocking(move || conv_repo::create(title))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let conversations = tokio::task::spawn_blocking(|| conv_repo::get_all())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let conversation = conversations
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create conversation".to_string())?;

    println!("[COMMAND] Created conversation: {}", conversation.id);
    Ok(conversation)
}

#[tauri::command]
pub async fn get_conversations() -> Result<Vec<Conversation>, String> {
    println!("[COMMAND] get_conversations called");
    let conversations = tokio::task::spawn_blocking(|| conv_repo::get_all())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    println!("[COMMAND] get_conversation called: id={}", id);
    let id_clone = id.clone();
    let result = tokio::task::spawn_blocking(move || conv_repo::get_by_id(&id_clone))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Conversation not found".to_string())?;
    Ok(result)
}

// AI Chat
#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    provider: String,
    conversation_id: String,
    user_message: String,
    capture_screenshot: bool,
) -> Result<(), String> {
    println!(
        "[COMMAND] send_message called: provider={}, conversation_id={}, capture_screenshot={}",
        provider, conversation_id, capture_screenshot
    );
    println!("[COMMAND] user_message: {}", user_message);

    if STREAMING.load(Ordering::SeqCst) {
        println!("[COMMAND] Already streaming, ignoring");
        return Err("Already streaming".to_string());
    }

    // Save user message immediately
    let conv_id_clone = conversation_id.clone();
    let user_msg_clone = user_message.clone();
    if let Err(e) = tokio::task::spawn_blocking(move || {
        msg_repo::create(conv_id_clone, MessageRole::User, user_msg_clone)
    })
    .await
    {
        println!("[DB] Error saving user message: {}", e);
    }

    STREAMING.store(true, Ordering::SeqCst);

    // Poem with markdown to stream over ~15 seconds
    let poem = r#"Here's a poem for you:

## The Code

In digital realms we write our fate,
With keyboard strokes we contemplate,
**Algorithms** dance and play,
*Variables* along the way.

```rust
fn main() {
    println!("Hello, World!");
}
```

- First we dream
- Then we code
- Finally we deploy

That's the way the *poem* goes..."#;

    let chars: Vec<char> = poem.chars().collect();
    let total_chars = chars.len();
    let duration_ms = 15000u64;
    let delay_per_char = duration_ms / total_chars as u64;

    let mut assistant_response = String::new();
    let completed = loop {
        if !STREAMING.load(Ordering::SeqCst) {
            println!("[COMMAND] Stream stopped by user");
            break false;
        }

        let chunk: String = chars
            .iter()
            .skip(assistant_response.len())
            .take(3)
            .collect();

        if chunk.is_empty() {
            break true;
        }

        assistant_response.push_str(&chunk);
        let _ = app.emit("chat-stream", chunk);

        tokio::time::sleep(tokio::time::Duration::from_millis(delay_per_char * 3)).await;
    };

    let _ = app.emit("chat-stream", "[DONE]");
    STREAMING.store(false, Ordering::SeqCst);

    if completed {
        println!("[COMMAND] Stream completed");
    } else {
        println!("[COMMAND] Stream stopped - saving partial response");
    }

    // Save assistant response (full or partial)
    let conv_id_final = conversation_id.clone();
    let assistant_final = assistant_response.clone();
    if let Err(e) = tokio::task::spawn_blocking(move || {
        msg_repo::create(conv_id_final, MessageRole::Assistant, assistant_final)
    })
    .await
    {
        println!("[DB] Error saving assistant message: {}", e);
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_stream() -> Result<(), String> {
    println!("[COMMAND] stop_stream called");
    STREAMING.store(false, Ordering::SeqCst);
    Ok(())
}
