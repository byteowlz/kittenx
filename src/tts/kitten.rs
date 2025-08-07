use crate::onnx::{KittenOnnx, AccelerationProvider};
use crate::tts::{phonemizer, text_cleaner::TextCleaner};
use crate::utils::{download_file, save_wav};
use anyhow::{Context, Result};
use ndarray::Array1;
use ndarray_npy::NpzReader;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct KittenTTS {
    model: Arc<Mutex<KittenOnnx>>,
    voices: HashMap<String, Array1<f32>>,
    text_cleaner: TextCleaner,
    available_voices: Vec<String>,
    sample_rate: u32,
}

impl KittenTTS {
    pub async fn new(model_dir: &Path) -> Result<Self> {
        Self::with_provider(model_dir, AccelerationProvider::Cpu).await
    }

    pub async fn with_provider(model_dir: &Path, provider: AccelerationProvider) -> Result<Self> {
        // Ensure model directory exists
        tokio::fs::create_dir_all(model_dir).await?;

        let config_path = model_dir.join("config.json");
        let model_path = model_dir.join("kitten_tts_nano_v0_1.onnx");
        let voices_path = model_dir.join("voices.npz");

        // Download files if they don't exist
        if !config_path.exists() {
            download_file(
                "https://huggingface.co/KittenML/kitten-tts-nano-0.1/resolve/main/config.json",
                &config_path,
            ).await?;
        }

        if !model_path.exists() {
            download_file(
                "https://huggingface.co/KittenML/kitten-tts-nano-0.1/resolve/main/kitten_tts_nano_v0_1.onnx",
                &model_path,
            ).await?;
        }

        if !voices_path.exists() {
            download_file(
                "https://huggingface.co/KittenML/kitten-tts-nano-0.1/resolve/main/voices.npz",
                &voices_path,
            ).await?;
        }

        // Load ONNX model with specified provider
        let model = Arc::new(Mutex::new(KittenOnnx::with_provider(model_path.to_str().unwrap(), provider)?));

        // Load voices
        let voices = Self::load_voices(&voices_path)?;

        let available_voices = vec![
            "expr-voice-2-m".to_string(),
            "expr-voice-2-f".to_string(),
            "expr-voice-3-m".to_string(),
            "expr-voice-3-f".to_string(),
            "expr-voice-4-m".to_string(),
            "expr-voice-4-f".to_string(),
            "expr-voice-5-m".to_string(),
            "expr-voice-5-f".to_string(),
        ];

        Ok(Self {
            model,
            voices,
            text_cleaner: TextCleaner::new(),
            available_voices,
            sample_rate: 24000,
        })
    }

    fn load_voices(voices_path: &Path) -> Result<HashMap<String, Array1<f32>>> {
        let file = File::open(voices_path)
            .context("Failed to open voices file")?;
        
        let mut npz = NpzReader::new(file)?;
        let mut voices = HashMap::new();

        // Try to load each voice
        let voice_names = [
            "expr-voice-2-m", "expr-voice-2-f", "expr-voice-3-m", "expr-voice-3-f",
            "expr-voice-4-m", "expr-voice-4-f", "expr-voice-5-m", "expr-voice-5-f"
        ];

        for voice_name in &voice_names {
            match npz.by_name::<ndarray::OwnedRepr<f32>, ndarray::Ix1>(voice_name) {
                Ok(voice_array) => {
                    voices.insert(voice_name.to_string(), voice_array);
                    println!("Loaded voice: {}", voice_name);
                }
                Err(_) => {
                    println!("Warning: Voice {} not found in NPZ file, using dummy", voice_name);
                    // Create a dummy voice embedding as fallback
                    voices.insert(voice_name.to_string(), Array1::zeros(256));
                }
            }
        }

        if voices.is_empty() {
            anyhow::bail!("No voices could be loaded from the NPZ file");
        }

        Ok(voices)
    }

    pub fn available_voices(&self) -> &[String] {
        &self.available_voices
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn generate(&self, text: &str, voice: &str, speed: f32) -> Result<Vec<f32>> {
        if !self.available_voices.contains(&voice.to_string()) {
            anyhow::bail!("Voice '{}' not available. Available voices: {:?}", voice, self.available_voices);
        }

        // Detect language and phonemize
        let language = phonemizer::detect_language(text).unwrap_or_else(|| "en-us".to_string());
        println!("Detected language: {}", language);

        // Convert text to phonemes
        let phonemes = phonemizer::text_to_phonemes_simple(text, &language)
            .unwrap_or_else(|_| {
                // Fallback to basic tokenization
                phonemizer::basic_tokenize(text).join(" ")
            });

        println!("Phonemes: {}", phonemes);

        // Convert phonemes to tokens
        let mut tokens = self.text_cleaner.clean(&phonemes);
        
        // Add start and end tokens
        tokens.insert(0, 0);
        tokens.push(0);

        println!("Tokens: {:?}", tokens);

        // Get voice embedding
        let voice_embedding = self.voices.get(voice)
            .ok_or_else(|| anyhow::anyhow!("Voice not found: {}", voice))?;

        // Run inference
        let input_ids = vec![tokens];
        let style = voice_embedding.to_vec();
        
        let output = {
            let mut model = self.model.lock().unwrap();
            model.infer(input_ids, style, speed)
                .context("ONNX inference failed")?
        };

        // Convert output to Vec<f32>
        let audio_data: Vec<f32> = output.iter().cloned().collect();
        
        // Trim audio (similar to Python implementation)
        let start_trim = 5000.min(audio_data.len());
        let end_trim = 10000.min(audio_data.len().saturating_sub(start_trim));
        let trimmed = if audio_data.len() > start_trim + end_trim {
            audio_data[start_trim..audio_data.len() - end_trim].to_vec()
        } else {
            audio_data
        };

        Ok(trimmed)
    }

    pub fn generate_to_file(&self, text: &str, voice: &str, speed: f32, output_path: &Path) -> Result<()> {
        let audio = self.generate(text, voice, speed)?;
        save_wav(&audio, self.sample_rate, output_path)?;
        println!("Audio saved to {}", output_path.display());
        Ok(())
    }
}