use std::borrow::Cow;
use ndarray::{ArrayBase, IxDyn, OwnedRepr};
use ort::{
    session::{Session, SessionInputValue, SessionInputs, SessionOutputs, builder::SessionBuilder},
    value::{Tensor, Value},
};

#[cfg(feature = "cuda")]
use ort::execution_providers::cuda::CUDAExecutionProvider;
#[cfg(feature = "coreml")]
use ort::execution_providers::coreml::CoreMLExecutionProvider;
use ort::execution_providers::cpu::CPUExecutionProvider;
use anyhow::Result;

pub struct KittenOnnx {
    session: Option<Session>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum AccelerationProvider {
    Cpu,
    Cuda,
    CoreML,
    DirectML,
    TensorRT,
    ROCm,
    OpenVINO,
    OneDNN,
    WebGPU,
}

unsafe impl Send for KittenOnnx {}
unsafe impl Sync for KittenOnnx {}

impl KittenOnnx {
    pub fn new(model_path: &str) -> Result<Self> {
        Self::with_provider(model_path, AccelerationProvider::Cpu)
    }

    pub fn with_provider(model_path: &str, provider: AccelerationProvider) -> Result<Self> {
        let mut instance = KittenOnnx { session: None };
        instance.load_model_with_provider(model_path, provider)?;
        Ok(instance)
    }

    fn load_model(&mut self, model_path: &str) -> Result<()> {
        self.load_model_with_provider(model_path, AccelerationProvider::Cpu)
    }

    fn load_model_with_provider(&mut self, model_path: &str, provider: AccelerationProvider) -> Result<()> {
        let mut builder = SessionBuilder::new()?;

        // Configure execution providers based on the selected provider
        let builder = match provider {
            AccelerationProvider::Cpu => {
                println!("Using CPU execution provider");
                builder.with_execution_providers([CPUExecutionProvider::default().build()])?
            }
            #[cfg(feature = "cuda")]
            AccelerationProvider::Cuda => {
                println!("Using CUDA execution provider");
                builder.with_execution_providers([
                    CUDAExecutionProvider::default().build(),
                    CPUExecutionProvider::default().build(),
                ])?
            }
            #[cfg(feature = "coreml")]
            AccelerationProvider::CoreML => {
                println!("Using CoreML execution provider");
                builder.with_execution_providers([
                    CoreMLExecutionProvider::default().build(),
                    CPUExecutionProvider::default().build(),
                ])?
            }
            _ => {
                println!("Requested provider not available in this build, falling back to CPU");
                builder.with_execution_providers([CPUExecutionProvider::default().build()])?
            }
        };

        let session = builder.commit_from_file(model_path)?;
        self.session = Some(session);
        Ok(())
    }

    pub fn infer(
        &mut self,
        input_ids: Vec<Vec<i64>>,
        style: Vec<f32>,
        speed: f32,
    ) -> Result<ArrayBase<OwnedRepr<f32>, IxDyn>> {
        let session = self.session.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Session not initialized"))?;

        // Prepare input_ids tensor
        let shape = [input_ids.len(), input_ids[0].len()];
        let input_ids_flat: Vec<i64> = input_ids.into_iter().flatten().collect();
        let input_ids_tensor = Tensor::from_array((shape, input_ids_flat))?;
        let input_ids_value = SessionInputValue::Owned(Value::from(input_ids_tensor));

        // Prepare style tensor
        let style_shape = [1, style.len()];
        let style_tensor = Tensor::from_array((style_shape, style))?;
        let style_value = SessionInputValue::Owned(Value::from(style_tensor));

        // Prepare speed tensor
        let speed_tensor = Tensor::from_array(([1], vec![speed]))?;
        let speed_value = SessionInputValue::Owned(Value::from(speed_tensor));

        let inputs: Vec<(Cow<str>, SessionInputValue)> = vec![
            (Cow::Borrowed("input_ids"), input_ids_value),
            (Cow::Borrowed("style"), style_value),
            (Cow::Borrowed("speed"), speed_value),
        ];

        let outputs: SessionOutputs = session.run(SessionInputs::from(inputs))?;
        let output_tensor = outputs[0].try_extract_tensor::<f32>()?;
        
        // Convert the tensor data to ndarray
        let (shape, data) = output_tensor;
        let dims: Vec<usize> = shape.iter().map(|&x| x as usize).collect();
        let output = ArrayBase::from_shape_vec(IxDyn(&dims), data.to_vec())?;

        Ok(output)
    }
}