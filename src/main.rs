use anyhow::Result;
use clap::{Parser, Subcommand};
use kittenx::KittenTTS;
use kittenx::onnx::AccelerationProvider;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kittenx")]
#[command(about = "KittenX - Pure Rust TTS with GPU acceleration")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate speech from text
    Generate {
        /// Text to synthesize
        #[arg(short, long)]
        text: String,
        
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        
        /// Voice to use for synthesis
        #[arg(short, long, default_value = "expr-voice-5-m")]
        voice: String,
        
        /// Speech speed (1.0 = normal)
        #[arg(short, long, default_value = "1.0")]
        speed: f32,
        
        /// Model directory path
        #[arg(short, long, default_value = "./models")]
        model_dir: PathBuf,
        
        /// Acceleration provider to use
        #[arg(short = 'p', long, default_value = "cpu")]
        provider: AccelerationProvider,
    },
    
    /// List available voices
    ListVoices {
        /// Model directory path
        #[arg(short, long, default_value = "./models")]
        model_dir: PathBuf,
        
        /// Acceleration provider to use
        #[arg(short = 'p', long, default_value = "cpu")]
        provider: AccelerationProvider,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { text, output, voice, speed, model_dir, provider } => {
            println!("Loading KittenTTS model...");
            let tts = KittenTTS::with_provider(&model_dir, provider).await?;
            
            println!("Generating speech for: \"{}\"", text);
            println!("Using voice: {}", voice);
            println!("Speed: {}", speed);
            
            tts.generate_to_file(&text, &voice, speed, &output)?;
        }
        
        Commands::ListVoices { model_dir, provider } => {
            println!("Loading KittenTTS model...");
            let tts = KittenTTS::with_provider(&model_dir, provider).await?;
            
            println!("Available voices:");
            for voice in tts.available_voices() {
                println!("  - {}", voice);
            }
        }
    }
    
    Ok(())
}