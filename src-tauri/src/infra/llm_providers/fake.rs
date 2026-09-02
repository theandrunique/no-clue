use std::time::Instant;

use async_trait::async_trait;
use futures_util::Stream;

use crate::domain::chats::{
    LlmChatCompletionChunk, LlmChatCompletionRequest, LlmChatStream, LlmProvider, LlmSettings,
};

pub struct FakeProvider;

impl FakeProvider {
    fn duration_ms(model_settings: &LlmSettings) -> u64 {
        match model_settings {
            LlmSettings::Fake { duration } => parse_duration_secs(duration) * 1000,
            _ => 15_000,
        }
    }
}

fn parse_duration_secs(value: &str) -> u64 {
    value
        .strip_suffix('s')
        .and_then(|seconds| seconds.parse().ok())
        .unwrap_or(15)
}

struct FakeStream {
    chars: Vec<char>,
    pos: usize,
    delay_ms: u64,
    next_time: Option<Instant>,
}

impl FakeStream {
    fn new(chars: Vec<char>, delay_ms: u64) -> Self {
        Self {
            chars,
            pos: 0,
            delay_ms,
            next_time: None,
        }
    }
}

impl Stream for FakeStream {
    type Item = Result<LlmChatCompletionChunk, anyhow::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = &mut *self;

        let now = Instant::now();

        if this.next_time.is_none() {
            this.next_time = Some(now + std::time::Duration::from_millis(this.delay_ms));
        }

        if let Some(next) = this.next_time {
            if now < next {
                cx.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }

            this.next_time = Some(now + std::time::Duration::from_millis(this.delay_ms));

            if this.pos >= this.chars.len() {
                return std::task::Poll::Ready(None);
            }

            let end = (this.pos + 3).min(this.chars.len());
            let content: String = this.chars[this.pos..end].iter().collect();
            this.pos = end;

            let is_finish = this.pos >= this.chars.len();
            std::task::Poll::Ready(Some(Ok(LlmChatCompletionChunk {
                content,
                is_finish,
                usage: None,
            })))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

#[async_trait]
impl LlmProvider for FakeProvider {
    async fn stream_chat_completion(
        &self,
        request: LlmChatCompletionRequest,
    ) -> Result<LlmChatStream, anyhow::Error> {
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

That's the way the *poem* goes."#;

        let chars: Vec<char> = poem.chars().collect();
        let total_chars = chars.len();
        let duration_ms = Self::duration_ms(&request.model);
        let delay_ms = if total_chars > 0 {
            duration_ms / (total_chars as u64 / 3)
        } else {
            100
        };

        Ok(Box::new(FakeStream::new(chars, delay_ms)))
    }
}
