use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::{
    domain::transcriptions::actor::{TranscriptionActorCommand, WorkerEvent},
    domain::transcriptions::{AudioCaptureConfig, SttProvider},
    infra::audio_capture::start_capture_pipeline,
};

pub async fn run_transcription(
    mut provider: Box<dyn SttProvider>,
    config: AudioCaptureConfig,
    actor_tx: mpsc::Sender<TranscriptionActorCommand>,
    ct: CancellationToken,
) {
    let audio = match start_capture_pipeline(&config, ct.child_token()) {
        Ok(stream) => stream,
        Err(e) => {
            tracing::error!(error = %e, "Failed to start audio capture");
            let _ = actor_tx
                .send(TranscriptionActorCommand::WorkerUpdate(WorkerEvent::Error(
                    "Failed to start audio capture".to_string(),
                )))
                .await;
            return;
        }
    };

    let mut results = match provider.transcribe(audio).await {
        Ok(stream) => stream,
        Err(e) => {
            tracing::error!(error = %e, "Failed to start transcription session");
            let _ = actor_tx
                .send(TranscriptionActorCommand::WorkerUpdate(WorkerEvent::Error(
                    "Failed to start transcription session".to_string(),
                )))
                .await;
            return;
        }
    };

    let _ = actor_tx
        .send(TranscriptionActorCommand::WorkerUpdate(
            WorkerEvent::Started,
        ))
        .await;

    loop {
        tokio::select! {
            result = results.next() => {
                match result {
                    Some(r) => {
                        let _ = actor_tx.send(TranscriptionActorCommand::WorkerUpdate(WorkerEvent::Result(r))).await;
                    },
                    None => break,
                }
            }
            _ = ct.cancelled() => {
                break;
            }
        }
    }

    let _ = actor_tx
        .send(TranscriptionActorCommand::WorkerUpdate(
            WorkerEvent::Finished,
        ))
        .await;
}
