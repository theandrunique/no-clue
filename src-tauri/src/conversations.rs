use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

static STREAMING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// Conversation management
#[tauri::command]
pub async fn create_conversation() -> Result<Conversation, String> {
    println!("[COMMAND] create_conversation called");

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as i64;

    let conversation = Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        title: "New conversation".to_string(),
        created_at: now,
        updated_at: now,
    };

    println!("[COMMAND] Created conversation: {:?}", conversation);
    Ok(conversation)
}

#[tauri::command]
pub async fn get_conversations() -> Result<Vec<Conversation>, String> {
    println!("[COMMAND] get_conversations called");
    // TODO: Read from SQLite
    Ok(vec![])
}

#[tauri::command]
pub async fn get_conversation(id: String) -> Result<Conversation, String> {
    println!("[COMMAND] get_conversation called: id={}", id);
    // TODO: Read from SQLite
    Err("Not implemented".to_string())
}

// AI Chat - streams a poem with markdown
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

    STREAMING.store(true, Ordering::SeqCst);

    // Poem with markdown to stream over ~3 seconds
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
    let duration_ms = 15000u64; // 15 seconds for slower streaming
    let delay_per_char = duration_ms / total_chars as u64;

    for chunk in chars.chunks(3) {
        if !STREAMING.load(Ordering::SeqCst) {
            println!("[COMMAND] Stream stopped by user");
            break;
        }

        let chunk_str: String = chunk.iter().collect();
        let _ = app.emit("chat-stream", chunk_str);

        tokio::time::sleep(tokio::time::Duration::from_millis(delay_per_char * 3)).await;
    }

    let _ = app.emit("chat-stream", "[DONE]");
    STREAMING.store(false, Ordering::SeqCst);
    println!("[COMMAND] Stream completed");

    Ok(())
}

#[tauri::command]
pub async fn stop_stream() -> Result<(), String> {
    println!("[COMMAND] stop_stream called");
    STREAMING.store(false, Ordering::SeqCst);
    Ok(())
}
