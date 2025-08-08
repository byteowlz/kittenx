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

/// Compute frame-wise RMS energy
fn rms_frames(audio: &[f32], frame_len: usize, hop_len: usize) -> Vec<f32> {
    if audio.is_empty() || frame_len == 0 { return vec![]; }
    let mut rms = Vec::new();
    let mut start = 0usize;
    while start < audio.len() {
        let end = (start + frame_len).min(audio.len());
        let frame = &audio[start..end];
        let mut sum = 0.0f64;
        for &s in frame { sum += (s as f64) * (s as f64); }
        let mean = if frame.is_empty() { 0.0 } else { sum / frame.len() as f64 };
        rms.push((mean.sqrt()) as f32);
        if start + hop_len >= audio.len() { break; }
        start += hop_len;
    }
    rms
}

/// Trim leading/trailing silence using an RMS threshold (librosa-like)
/// - top_db: threshold in dB relative to max RMS (e.g., 40.0)
/// - frame_ms/hop_ms: analysis window and hop in milliseconds
/// - min_silence_ms: require at least this much silence at head/tail to trim
/// - end_padding_ms: keep some padding at the end to avoid cutting off last phonemes
pub fn trim_silence(
    audio: &[f32],
    sample_rate: u32,
    top_db: f32,
    frame_ms: f32,
    hop_ms: f32,
    min_silence_ms: f32,
    end_padding_ms: f32,
) -> Vec<f32> {
    if audio.is_empty() { return vec![]; }
    let sr = sample_rate as f32;
    let frame_len = ((frame_ms / 1000.0) * sr).max(1.0) as usize;
    let hop_len = ((hop_ms / 1000.0) * sr).max(1.0) as usize;
    let min_silence_frames = ((min_silence_ms / 1000.0) * sr / hop_len as f32).ceil() as usize;

    let env = rms_frames(audio, frame_len, hop_len);
    if env.is_empty() { return audio.to_vec(); }

    let &ref_rms = env.iter().fold(&0.0f32, |a, b| if b > a { b } else { a });
    if ref_rms <= 0.0 { return audio.to_vec(); }

    // Convert dB threshold relative to ref
    let threshold = ref_rms * 10f32.powf(-top_db / 20.0);

    // Find first index where we have a run of non-silence frames
    let mut start_frame = 0usize;
    let mut run = 0usize;
    for (i, &e) in env.iter().enumerate() {
        if e >= threshold { run += 1; } else { run = 0; }
        if run >= 2 { // require a couple frames above threshold
            start_frame = i.saturating_sub(1);
            break;
        }
    }

    // Find last index where we have a run of non-silence frames
    let mut end_frame = env.len().saturating_sub(1);
    run = 0;
    for i in (0..env.len()).rev() {
        let e = env[i];
        if e >= threshold { run += 1; } else { run = 0; }
        if run >= 2 {
            end_frame = (i + 1).min(env.len().saturating_sub(1));
            break;
        }
    }

    // Convert frames to sample indices
    let mut start_sample = start_frame.saturating_mul(hop_len);
    let mut end_sample = ((end_frame + 1) * hop_len).min(audio.len());

    // Only trim if we have enough silence duration before/after
    // Head
    let head_silence_end = start_sample;
    let head_silence_frames = (head_silence_end / hop_len) as usize;
    if head_silence_frames < min_silence_frames { start_sample = 0; }

    // Tail
    let tail_silence_start = end_sample;
    let tail_silence_frames = ((audio.len().saturating_sub(tail_silence_start)) / hop_len) as usize;
    if tail_silence_frames < min_silence_frames { end_sample = audio.len(); }

    // Keep some padding at the end to be safe
    let pad_samples = ((end_padding_ms / 1000.0) * sr) as usize;
    end_sample = (end_sample + pad_samples).min(audio.len());

    // Also keep a tiny padding at the start to avoid abrupt start
    let start_pad = ((5.0 / 1000.0) * sr) as usize; // 5ms
    start_sample = start_sample.saturating_sub(start_pad);

    if start_sample >= end_sample { return audio.to_vec(); }

    let mut out = audio[start_sample..end_sample].to_vec();

    // Apply gentle fades to avoid clicks
    apply_fade_in_out(&mut out, sample_rate, 5.0, 10.0);
    out
}

/// Apply linear fade in/out in milliseconds
pub fn apply_fade_in_out(audio: &mut [f32], sample_rate: u32, fade_in_ms: f32, fade_out_ms: f32) {
    if audio.is_empty() { return; }
    let sr = sample_rate as f32;
    let fade_in_samples = ((fade_in_ms / 1000.0) * sr).max(0.0) as usize;
    let fade_out_samples = ((fade_out_ms / 1000.0) * sr).max(0.0) as usize;

    let n = audio.len();

    for i in 0..fade_in_samples.min(n) {
        let gain = (i as f32) / (fade_in_samples as f32);
        audio[i] *= gain;
    }
    for i in 0..fade_out_samples.min(n) {
        let idx = n - 1 - i;
        let gain = (i as f32) / (fade_out_samples as f32);
        audio[idx] *= 1.0 - gain;
    }
}
