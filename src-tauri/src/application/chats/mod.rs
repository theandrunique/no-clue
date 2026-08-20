use std::sync::LazyLock;

use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

mod cancel_generation;
mod chat_regenerate;
mod chat_send;
mod generation;

pub use cancel_generation::stop_stream;
pub use chat_regenerate::retry_generation;
pub use chat_send::send_message;

static SESSION: LazyLock<Mutex<Option<CancellationToken>>> = LazyLock::new(|| Mutex::new(None));
