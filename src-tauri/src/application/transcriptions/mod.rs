mod update_transcription_session;
mod start_transcription;
mod stop_transcription;

mod process;

pub use update_transcription_session::update_transcription_session;
pub use start_transcription::start_transcription;
pub use stop_transcription::stop_transcription;

use std::{sync::LazyLock};

use tauri::{AppHandle, Emitter};
use tokio::{sync::Mutex, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::domain::events;

struct TranscriptionSession {
    cancellation_token: CancellationToken,
    task: JoinHandle<()>,
}

static SESSION: LazyLock<Mutex<Option<TranscriptionSession>>> = LazyLock::new(|| Mutex::new(None));
static CURRENT_CONVERSATION_ID: LazyLock<Mutex<Option<Uuid>>> = LazyLock::new(|| Mutex::new(None));

async fn finish(app: AppHandle) {
    *SESSION.lock().await = None;
    let _ = app.emit(events::TRANSCRIPTION_STOPPED, ());
}
