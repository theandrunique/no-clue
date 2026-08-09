// TODO: Replace simple decimation with high-quality resampling via rubato crate
// Currently using step_by() decimation which is simple but may introduce aliasing.
// rubato provides polynomial interpolation for better quality resampling.

pub fn f32_to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * 32767.0) as i16
}

pub fn interleave_stereo(left: &[i16], right: &[i16]) -> Vec<u8> {
    let min_len = left.len().min(right.len());
    let mut result = Vec::with_capacity(min_len * 4);
    for i in 0..min_len {
        result.extend_from_slice(&left[i].to_le_bytes());
        result.extend_from_slice(&right[i].to_le_bytes());
    }
    result
}

pub fn interleave_with_silence(
    left: &[i16],
    use_silence_left: bool,
    right: &[i16],
    use_silence_right: bool,
) -> Vec<u8> {
    let len = left.len().max(right.len());
    let mut result = Vec::with_capacity(len * 4);

    for i in 0..len {
        let left_sample = if use_silence_left || i >= left.len() {
            0i16
        } else {
            left[i]
        };
        let right_sample = if use_silence_right || i >= right.len() {
            0i16
        } else {
            right[i]
        };
        result.extend_from_slice(&left_sample.to_le_bytes());
        result.extend_from_slice(&right_sample.to_le_bytes());
    }
    result
}

pub fn decimate(samples: &[i16], ratio: usize) -> Vec<i16> {
    if ratio <= 1 {
        return samples.to_vec();
    }
    samples.iter().step_by(ratio).copied().collect()
}

pub fn decimate_to_target(input: &[i16], source_rate: u32, target_rate: u32) -> Vec<i16> {
    if source_rate == target_rate {
        return input.to_vec();
    }
    let ratio = source_rate / target_rate;
    decimate(input, ratio as usize)
}

#[allow(dead_code)]
pub struct AudioProcessor {
    decimation_ratio: usize,
    chunk_samples: usize,
    metrics: ProcessorMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessorMetrics {
    pub chunks_processed: u64,
    pub bytes_sent: u64,
    pub system_chunks: u64,
    pub mic_chunks: u64,
    pub mixed_chunks: u64,
    pub buffer_underruns: u64,
}

impl AudioProcessor {
    pub fn new(source_rate: u32, target_rate: u32, chunk_duration_ms: u32) -> Self {
        let decimation_ratio = if source_rate != target_rate {
            (source_rate / target_rate) as usize
        } else {
            1
        };
        let chunk_samples = ((source_rate / 1000) * chunk_duration_ms) as usize;

        Self {
            decimation_ratio,
            chunk_samples,
            metrics: ProcessorMetrics::default(),
        }
    }

    pub fn chunk_samples(&self) -> usize {
        self.chunk_samples
    }

    #[allow(dead_code)]
    pub fn process_chunk(&mut self, system: Option<&[i16]>, mic: Option<&[i16]>) -> Vec<u8> {
        let has_system = system.is_some();
        let has_mic = mic.is_some();

        match (has_system, has_mic) {
            (true, true) => {
                self.metrics.mixed_chunks += 1;
                let sys_chunk = system.unwrap();
                let mic_chunk = mic.unwrap();

                let dec_sys = decimate(sys_chunk, self.decimation_ratio);
                let dec_mic = decimate(mic_chunk, self.decimation_ratio);

                let min_len = dec_sys.len().min(dec_mic.len());
                let mut interleaved = Vec::with_capacity(min_len * 4);
                for i in 0..min_len {
                    interleaved.extend_from_slice(&dec_sys[i].to_le_bytes());
                    interleaved.extend_from_slice(&dec_mic[i].to_le_bytes());
                }
                self.metrics.bytes_sent += interleaved.len() as u64;
                interleaved
            }
            (true, false) => {
                self.metrics.system_chunks += 1;
                let sys_chunk = system.unwrap();
                let dec_sys = decimate(sys_chunk, self.decimation_ratio);

                let mut interleaved = Vec::with_capacity(dec_sys.len() * 4);
                for sample in dec_sys {
                    interleaved.extend_from_slice(&sample.to_le_bytes());
                    interleaved.extend_from_slice(&0i16.to_le_bytes());
                }
                self.metrics.bytes_sent += interleaved.len() as u64;
                interleaved
            }
            (false, true) => {
                self.metrics.mic_chunks += 1;
                let mic_chunk = mic.unwrap();
                let dec_mic = decimate(mic_chunk, self.decimation_ratio);

                let mut interleaved = Vec::with_capacity(dec_mic.len() * 4);
                for sample in dec_mic {
                    interleaved.extend_from_slice(&0i16.to_le_bytes());
                    interleaved.extend_from_slice(&sample.to_le_bytes());
                }
                self.metrics.bytes_sent += interleaved.len() as u64;
                interleaved
            }
            (false, false) => {
                self.metrics.buffer_underruns += 1;
                Vec::new()
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_metrics(&self) -> ProcessorMetrics {
        self.metrics.clone()
    }

    #[allow(dead_code)]
    pub fn reset_metrics(&mut self) {
        self.metrics = ProcessorMetrics::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_to_i16() {
        assert_eq!(f32_to_i16(0.0), 0);
        assert_eq!(f32_to_i16(1.0), 32767);
        assert_eq!(f32_to_i16(-1.0), -32767);
        assert_eq!(f32_to_i16(0.5), 16383);
    }

    #[test]
    fn test_interleave_stereo() {
        let left = vec![1i16, 2, 3];
        let right = vec![10i16, 20, 30];
        let result = interleave_stereo(&left, &right);
        assert_eq!(result.len(), 12);
    }

    #[test]
    fn test_decimate() {
        let samples: Vec<i16> = (0..12).map(|i| i).collect();
        let decimated = decimate(&samples, 3);
        assert_eq!(decimated, vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_audio_processor_mixed() {
        let mut processor = AudioProcessor::new(48000, 16000, 100);
        let system: Vec<i16> = (0..4800).map(|i| i as i16).collect();
        let mic: Vec<i16> = (0..4800).map(|i| i as i16).collect();

        let output = processor.process_chunk(Some(&system), Some(&mic));

        assert!(!output.is_empty());
        let metrics = processor.get_metrics();
        assert_eq!(metrics.mixed_chunks, 1);
    }

    #[test]
    fn test_audio_processor_system_only() {
        let mut processor = AudioProcessor::new(48000, 16000, 100);
        let system: Vec<i16> = (0..4800).map(|i| i as i16).collect();

        let output = processor.process_chunk(Some(&system), None);

        assert!(!output.is_empty());
        let metrics = processor.get_metrics();
        assert_eq!(metrics.system_chunks, 1);
    }
}
