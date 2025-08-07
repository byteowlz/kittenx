use anyhow::Result;
use hound::{WavSpec, WavWriter};
use std::path::Path;

pub fn save_wav(audio: &[f32], sample_rate: u32, path: &Path) -> Result<()> {
    // Use float32 format like Python soundfile to avoid quantization distortion
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)?;

    for &sample in audio {
        // Clamp to valid range but keep as float32
        let sample_f32 = sample.clamp(-1.0, 1.0);
        writer.write_sample(sample_f32)?;
    }

    writer.finalize()?;
    Ok(())
}

// Alternative function for 16-bit output if needed
pub fn save_wav_16bit(audio: &[f32], sample_rate: u32, path: &Path) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;

    for &sample in audio {
        let sample_i16 = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
        writer.write_sample(sample_i16)?;
    }

    writer.finalize()?;
    Ok(())
}