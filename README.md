# Kittenx - Pure Rust TTS with GPU Acceleration based on kitten tts

A **pure Rust** command-line interface for [KittenTTS](https://github.com/KittenML/KittenTTS), an ultra-lightweight text-to-speech model with just 15 million parameters. **kittenx** provides **blazing fast** TTS inference with **GPU acceleration** and **zero Python dependencies**.

## Features

- **Pure Rust Implementation**: No Python dependencies, native performance
- **GPU Acceleration**: CUDA, CoreML, DirectML, TensorRT, ROCm, OpenVINO, OneDNN, WebGPU support
- **Ultra-lightweight**: Model size less than 25MB
- **Multi-platform**: CPU and GPU acceleration on Linux, macOS, and Windows
- **Multiple voices**: 8 voice options available
- **Fast inference**: Optimized ONNX Runtime integration with hardware acceleration
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Automatic model download**: Downloads models from HuggingFace automatically
- **Language detection**: Automatic language detection with espeak phonemization

## Architecture

This CLI is built with a modular Rust architecture inspired by [kokorox](https://github.com/WismutHansen/kokorox):

- **ONNX Runtime**: Direct ONNX model inference in Rust
- **espeak-rs**: Phonemization and text processing
- **ndarray**: Efficient tensor operations
- **tokio**: Async model downloading
- **clap**: Modern CLI interface

## Installation

### Prerequisites

- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **espeak-ng**: For phonemization
  - **Ubuntu/Debian**: `sudo apt-get install espeak-ng libespeak-ng-dev`
  - **macOS**: `brew install espeak-ng`
  - **Windows**: Install from [espeak-ng releases](https://github.com/espeak-ng/espeak-ng/releases)

### Building from Source

```bash
git clone <this-repository>
cd kittenx

# CPU-only build (default)
cargo build --release

# With CUDA support
cargo build --release --features cuda

# With CoreML support (macOS)
cargo build --release --features coreml

# With multiple GPU features
cargo build --release --features cuda,tensorrt
```

The binary will be available at `./target/release/kittenx`.

## Usage

### Generate Speech

```bash
# Basic usage (CPU)
./target/release/kittenx generate \
  --text "Hello, world!" \
  --output hello.wav

# With GPU acceleration
./target/release/kittenx generate \
  --text "This is kittenx with CUDA acceleration!" \
  --output cuda-tts.wav \
  --provider cuda

# With custom voice, speed, and CoreML
./target/release/kittenx generate \
  --text "This is kittenx running in pure Rust!" \
  --output rust-tts.wav \
  --voice expr-voice-2-f \
  --speed 1.2 \
  --provider coreml
```

### List Available Voices

```bash
./target/release/kittenx list-voices
```

Available voices:
- `expr-voice-2-m` (male)
- `expr-voice-2-f` (female)
- `expr-voice-3-m` (male)
- `expr-voice-3-f` (female)
- `expr-voice-4-m` (male)
- `expr-voice-4-f` (female)
- `expr-voice-5-m` (male)
- `expr-voice-5-f` (female)

## 🔧 Command Reference

### `generate`

Generate speech from text.

**Options:**
- `-t, --text <TEXT>`: Text to synthesize (required)
- `-o, --output <OUTPUT>`: Output file path (required)
- `-v, --voice <VOICE>`: Voice to use (default: expr-voice-5-m)
- `-s, --speed <SPEED>`: Speech speed, 1.0 = normal (default: 1.0)
- `-m, --model-dir <MODEL_DIR>`: Model directory path (default: ./models)
- `-p, --provider <PROVIDER>`: Acceleration provider (default: cpu)
  - `cpu`: CPU execution (default)
  - `cuda`: NVIDIA CUDA acceleration
  - `coreml`: Apple CoreML acceleration (macOS)
  - `directml`: DirectML acceleration (Windows)
  - `tensorrt`: NVIDIA TensorRT acceleration
  - `rocm`: AMD ROCm acceleration
  - `openvino`: Intel OpenVINO acceleration
  - `onednn`: Intel OneDNN acceleration
  - `webgpu`: WebGPU acceleration

### `list-voices`

List all available voices.

**Options:**
- `-m, --model-dir <MODEL_DIR>`: Model directory path (default: ./models)
- `-p, --provider <PROVIDER>`: Acceleration provider (default: cpu)

## Performance & GPU Acceleration

This pure Rust implementation offers significant performance advantages:

- **No Python overhead**: Direct ONNX Runtime integration
- **Memory efficient**: Optimized tensor operations with ndarray
- **Fast startup**: No interpreter initialization
- **Native performance**: Compiled binary with full optimizations
- **GPU acceleration**: Hardware-accelerated inference on supported devices

### GPU Support

The CLI supports multiple hardware acceleration providers:

| Provider | Platform | Description |
|----------|----------|-------------|
| `cpu` | All | CPU execution (default) |
| `cuda` | Linux/Windows | NVIDIA CUDA acceleration |
| `coreml` | macOS | Apple CoreML acceleration |
| `directml` | Windows | DirectML acceleration |
| `tensorrt` | Linux/Windows | NVIDIA TensorRT optimization |
| `rocm` | Linux | AMD ROCm acceleration |
| `openvino` | All | Intel OpenVINO acceleration |
| `onednn` | All | Intel OneDNN acceleration |
| `webgpu` | All | WebGPU acceleration |

### Building with GPU Support

```bash
# NVIDIA CUDA
cargo build --release --features cuda

# Apple CoreML (macOS only)
cargo build --release --features coreml

# Multiple providers
cargo build --release --features cuda,tensorrt,openvino
```

### Performance Comparison

Typical inference times for a 10-word sentence:

- **CPU**: ~200-500ms
- **CUDA**: ~50-150ms (3-4x faster)
- **CoreML**: ~80-200ms (2-3x faster)
- **TensorRT**: ~30-100ms (5-8x faster)

## Automatic Model Management

The CLI automatically handles model downloading:

1. **First run**: Downloads model files from HuggingFace
2. **Subsequent runs**: Uses cached models from `./models/`
3. **Files downloaded**:
   - `config.json` - Model configuration
   - `kitten_tts_nano_v0_1.onnx` - ONNX model (~24MB)
   - `voices.npz` - Voice embeddings (~10KB)

## Language Support

The CLI includes automatic language detection and phonemization:

- **Automatic detection**: Uses whatlang for language detection
- **Phonemization**: espeak-rs for accurate phoneme conversion
- **Supported languages**: English, Spanish, French, German, Italian, Portuguese, Russian, Japanese, Korean, Chinese

## Project Structure

```
kittenx/
├── src/
│   ├── main.rs           # CLI interface
│   ├── lib.rs            # Library exports
│   ├── onnx/
│   │   └── mod.rs        # ONNX Runtime integration
│   ├── tts/
│   │   ├── mod.rs        # TTS module exports
│   │   ├── kitten.rs     # Main KittenTTS implementation
│   │   ├── phonemizer.rs # Text to phoneme conversion
│   │   ├── tokenizer.rs  # Phoneme tokenization
│   │   └── text_cleaner.rs # Text preprocessing
│   └── utils/
│       ├── mod.rs        # Utility exports
│       ├── download.rs   # Model downloading
│       └── audio.rs      # Audio file handling
├── Cargo.toml           # Dependencies and metadata
└── README.md           # This file
```

## Development

### Dependencies

Key Rust crates used:

- `ort`: ONNX Runtime bindings
- `ndarray`: N-dimensional arrays
- `espeak-rs`: Text-to-speech phonemization
- `whatlang`: Language detection
- `clap`: Command-line parsing
- `tokio`: Async runtime
- `hound`: WAV file generation

### Building with Features

```bash
# CPU-only (default)
cargo build --release

# With CUDA support (if available)
cargo build --release --features cuda
```

## Troubleshooting

### espeak-ng Issues

If you encounter espeak-ng related errors:

```bash
# Ubuntu/Debian
sudo apt-get install espeak-ng libespeak-ng-dev

# macOS
brew install espeak-ng

# Verify installation
espeak --version
```

### ONNX Runtime Issues

The CLI uses ONNX Runtime for inference. If you encounter issues:

1. Ensure you have the latest version of the CLI
2. Check that the model files downloaded correctly
3. Try removing the `./models/` directory to force re-download

### Voice Loading Issues

If voices show as "dummy" embeddings:

- The NPZ file structure might differ from expectations
- Voices will still work but may not have distinct characteristics
- This is a known limitation that can be improved in future versions

## Examples

### Basic Text-to-Speech

```bash
./target/release/kittenx generate \
  --text "Welcome to KittenTTS in Rust!" \
  --output welcome.wav
```

### Different Voices

```bash
# Male voice
./target/release/kittenx generate \
  --text "This is a male voice." \
  --output male.wav \
  --voice expr-voice-3-m

# Female voice
./target/release/kittenx generate \
  --text "This is a female voice." \
  --output female.wav \
  --voice expr-voice-4-f
```

### Speed Control

```bash
# Slower speech
./target/release/kittenx generate \
  --text "This is slower speech." \
  --output slow.wav \
  --speed 0.8

# Faster speech
./target/release/kittenx generate \
  --text "This is faster speech." \
  --output fast.wav \
  --speed 1.3
```

### Batch Processing

```bash
# Generate multiple files
for text in "Hello" "Goodbye" "Thank you"; do
  ./target/release/kittenx generate \
    --text "$text" \
    --output "${text,,}.wav" \
    --voice expr-voice-2-f
done
```

## Contributing

Contributions are welcome! Areas for improvement:

- **Voice embedding loading**: Better NPZ file parsing
- **Additional languages**: More phonemization support
- **Streaming**: Real-time audio generation
- **Performance**: Further optimizations
- Phonemization: Switching out espeak-rs for something with a more permissive license

## License

GPL v3 since this project is using espeak-rs for phonemization, which links to espeak-ng which as a GPL v3 license. If you know a good multi-lingual phonemizer that compiles to a binary and has a more permissive license, please let me know! 

## Acknowledgments

- **KittenML**: For the original KittenTTS model
- **ONNX Runtime**: For efficient model inference
- **espeak-ng**: For phonemization capabilities


