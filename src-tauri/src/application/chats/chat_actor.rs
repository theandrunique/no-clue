use tauri::AppHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub trait ChatActorOutpur: Send + Sync + 'static {
    fn on_error(&self, error: String);
}

pub enum ChatActorState {
    Idle,
    Generation {
        cancel_token: CancellationToken
    },
}

pub enum ChatActorCommand {
    Complete {
        provider: String,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message: String,
    },
    Regenerate {
        provider: String,
        capture_screenshot: bool,
        system_prompt_id: Option<Uuid>,
        user_message_id: Uuid,
    },
    StopCompletion,
}

pub struct ChatActor {
    app: AppHandle,
    conversation_id: Uuid,
    state: ChatActorState,
}
