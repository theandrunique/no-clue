mod deepgram;
mod fake;

pub use deepgram::deepgram_descriptor;
pub use fake::fake_stt_descriptor;

use crate::domain::stt::{SttProvider, SttProviderSettings};

pub fn create_stt_provider(settings: &SttProviderSettings) -> Result<Box<dyn SttProvider>, String> {
    match settings {
        SttProviderSettings::Fake => Ok(Box::new(fake::FakeSttProvider::new())),
        SttProviderSettings::Deepgram {
            api_key,
            language,
            model,
        } => Ok(Box::new(deepgram::DeepgramProvider::new(
            api_key.clone(),
            language.clone(),
            model.clone(),
        ))),
    }
}
