use crate::db::{self, Conversation};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

static STREAMING: AtomicBool = AtomicBool::new(false);

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    println!("[COMMAND] create_conversation called");

    let title = "New conversation".to_string();
    let _id = db::create_conversation_db(title).await?;

    let conversations = db::get_conversations_db().await?;
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
    let conversations = db::get_conversations_db().await?;
    Ok(conversations)
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    println!("[COMMAND] get_conversation called: id={}", id);
    db::get_conversation_db(id)
        .await?
        .ok_or_else(|| "Conversation not found".to_string())
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
    if let Err(e) =
        db::save_message(conversation_id.clone(), "user".to_string(), user_message).await
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
    if let Err(e) =
        db::save_message(conversation_id, "assistant".to_string(), assistant_response).await
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
