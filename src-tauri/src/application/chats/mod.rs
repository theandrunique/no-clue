use std::sync::LazyLock;

use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

mod cancel_generation;
mod chat_regenerate;
mod chat_send;
mod generation;
mod get_messages;
mod llm_providers;

pub use cancel_generation::stop_stream;
pub use chat_regenerate::retry_generation;
pub use chat_send::send_message;
pub use get_messages::get_messages;
pub use llm_providers::{
    get_llm_provider_settings, get_llm_providers, get_model_info, save_llm_provider_settings,
};

static SESSION: LazyLock<Mutex<Option<CancellationToken>>> = LazyLock::new(|| Mutex::new(None));
