pub mod tts;
pub mod onnx;
pub mod utils;

pub use tts::KittenTTS;
pub use onnx::AccelerationProvider;

use anyhow::Result;
use std::path::Path;

pub struct KittenXLib {
    tts: KittenTTS,
}

impl KittenXLib {
    pub async fn new<P: AsRef<Path>>(model_dir: P) -> Result<Self> {
        let tts = KittenTTS::new(model_dir.as_ref()).await?;
        Ok(Self { tts })
    }
    
    pub async fn with_provider<P: AsRef<Path>>(
        model_dir: P, 
        provider: AccelerationProvider
    ) -> Result<Self> {
        let tts = KittenTTS::with_provider(model_dir.as_ref(), provider).await?;
        Ok(Self { tts })
    }
    
    pub fn generate_speech(&self, text: &str, voice: &str, speed: f32) -> Result<Vec<f32>> {
        self.tts.generate(text, voice, speed)
    }
    
    pub fn generate_to_file<P: AsRef<Path>>(
        &self, 
        text: &str, 
        voice: &str, 
        speed: f32, 
        output_path: P
    ) -> Result<()> {
        self.tts.generate_to_file(text, voice, speed, output_path.as_ref())
    }
    
    pub fn available_voices(&self) -> Vec<String> {
        self.tts.available_voices().to_vec()
    }
}