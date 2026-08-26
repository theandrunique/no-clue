use std::collections::HashMap;

use tauri::AppHandle;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::application::chats::chat_handle::ChatHandle;

pub struct ChatRouter {
    app: AppHandle,
    cache: Mutex<HashMap<Uuid, ChatHandle>>,
}

impl ChatRouter {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_or_create(&self, conversation_id: Uuid) -> ChatHandle {
        let mut cache = self.cache.lock().await;

        cache
            .entry(conversation_id)
            .or_insert_with(|| ChatHandle::new(self.app.clone(), conversation_id))
            .clone()
    }
}
