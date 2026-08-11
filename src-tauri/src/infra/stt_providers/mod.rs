mod deepgram;
mod fake;

pub use deepgram::deepgram_descriptor;
pub use fake::fake_stt_descriptor;

use crate::domain::stt::{SttProvider, SttProviderSettings};

pub fn create_stt_provider(settings: &SttProviderSettings) -> Box<dyn SttProvider> {
    match settings {
        SttProviderSettings::Fake => Box::new(fake::FakeSttProvider::new()),
        SttProviderSettings::Deepgram {
            api_key,
            language,
            model,
        } => Box::new(deepgram::DeepgramProvider::new(
            api_key.clone(),
            language.clone(),
            model.clone(),
        )),
    }
}
