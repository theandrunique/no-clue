use std::pin::Pin;

use anyhow::Result;
use futures_util::Stream;
use serde::{Deserialize, Serialize};

use crate::domain::transcripts::AudioSource;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::{AudioInput as PlatformAudioInput, AudioStream as PlatformAudioStream};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{AudioInput as PlatformAudioInput, AudioStream as PlatformAudioStream};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{AudioInput as PlatformAudioInput, AudioStream as PlatformAudioStream};

mod pipeline;
pub use pipeline::start_capture_pipeline;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub(crate) fn list_input_devices() -> Result<Vec<AudioDevice>> {
    #[cfg(target_os = "macos")]
    return macos::get_input_devices();

    #[cfg(target_os = "windows")]
    return windows::get_input_devices();

    #[cfg(target_os = "linux")]
    return linux::get_input_devices();
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub(crate) fn list_output_devices() -> Result<Vec<AudioDevice>> {
    #[cfg(target_os = "macos")]
    return macos::get_output_devices();

    #[cfg(target_os = "windows")]
    return windows::get_output_devices();

    #[cfg(target_os = "linux")]
    return linux::get_output_devices();
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub(crate) fn list_input_devices() -> Result<Vec<AudioDevice>> {
    Ok(vec![])
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub(crate) fn list_output_devices() -> Result<Vec<AudioDevice>> {
    Ok(vec![])
}

pub struct AudioInput {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    inner: PlatformAudioInput,
}

impl AudioInput {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    pub fn system(device_id: Option<String>) -> Result<Self> {
        let inner = PlatformAudioInput::new(device_id, AudioSource::System)?;
        Ok(Self { inner })
    }

    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    pub fn microphone(device_id: Option<String>) -> Result<Self> {
        let inner = PlatformAudioInput::new(device_id, AudioSource::Microphone)?;
        Ok(Self { inner })
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    pub fn system(_device_id: Option<String>) -> Result<Self> {
        Err(anyhow::anyhow!(
            "AudioInput::system is not supported on this platform"
        ))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    pub fn microphone(_device_id: Option<String>) -> Result<Self> {
        Err(anyhow::anyhow!(
            "AudioInput::microphone is not supported on this platform"
        ))
    }

    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    pub fn stream(self) -> AudioStream {
        let inner = self.inner.stream();
        AudioStream { inner }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    pub fn stream(self) -> AudioStream {
        unimplemented!("AudioInput::stream is not supported on this platform")
    }
}

pub struct AudioStream {
    inner: PlatformAudioStream,
}

impl Stream for AudioStream {
    type Item = f32;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        {
            Pin::new(&mut self.inner).poll_next(cx)
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            std::task::Poll::Pending
        }
    }
}

impl AudioStream {
    pub fn sample_rate(&self) -> u32 {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        return self.inner.sample_rate();

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        0
    }
}
