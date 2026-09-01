mod deepgram;
mod fake;

use crate::domain::transcriptions::{SttProvider, SttProviderSettings};

pub fn create_stt_provider(settings: &SttProviderSettings) -> Box<dyn SttProvider> {
    match settings {
        SttProviderSettings::TestingProvider => Box::new(fake::FakeSttProvider::new()),
        SttProviderSettings::Deepgram { api_key } => Box::new(deepgram::DeepgramProvider::new(
            Some(api_key.clone()),
            Some("ru".to_string()),
            Some("nova-3".to_string()),
        )),
    }
}
