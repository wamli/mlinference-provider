#[cfg(feature = "tflite")]
mod tflite;
mod tract;

#[cfg(any(feature = "tflite", feature = "edgetpu"))]
pub use self::tflite::TfLiteEngine;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
pub use tract::{bytes_to_f32_vec, f32_array_to_bytes, TractEngine, TractSession};
use wasmcloud_interface_mlinference::{InferenceOutput, Tensor};

/// Graph (model number)
pub type Graph = u32;

/// GraphEncoding
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphEncoding {
    #[default]
    Onnx,
    TfLite,
    OpenVino,
    Tensorflow,
}

/// GraphExecutionContext
pub type GraphExecutionContext = u32;

/// ExecutionTarget
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionTarget {
    #[default]
    Cpu,
    Gpu,
    Tpu,
}

impl Default for Box<dyn InferenceEngine + Send + Sync> {
    fn default() -> Box<dyn InferenceEngine + Send + Sync>
    where
        Self: Sized,
    {
        Box::new(<TractEngine as Default>::default())
    }
}

/// InferenceEngine
#[async_trait]
pub trait InferenceEngine {
    async fn load(&self, model: &[u8]) -> InferenceResult<Graph>;

    async fn init_execution_context(
        &self,
        graph: Graph,
        target: &ExecutionTarget,
        encoding: &GraphEncoding,
    ) -> InferenceResult<GraphExecutionContext>;

    async fn set_input(
        &self,
        context: GraphExecutionContext,
        index: u32,
        tensor: &Tensor,
    ) -> InferenceResult<()>;

    async fn compute(&self, context: GraphExecutionContext) -> InferenceResult<()>;

    async fn get_output(
        &self,
        context: GraphExecutionContext,
        index: u32,
    ) -> InferenceResult<InferenceOutput>;

    async fn drop_model_state(&self, graph: &Graph, gec: &GraphExecutionContext);
}

/// InferenceResult
pub type InferenceResult<T> = Result<T, InferenceError>;

#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("runtime error")]
    RuntimeError,

    #[error("ONNX error")]
    OnnxError,

    #[error("Unsupported ExecutionTarget")]
    UnsupportedExecutionTarget,

    #[error("Invalid encoding")]
    InvalidEncodingError,

    #[error("Failed to build model from buffer")]
    FailedToBuildModelFromBuffer,

    #[error("Failed to get edge TPU context")]
    EdgeTPUAllocationError,

    #[error("Failed to get InterpreterBuilder")]
    InterpreterBuilderError,

    #[error("Interpreter build failed")]
    InterpreterBuildError,

    #[error("Interpreter invocation failed")]
    InterpreterInvocationError,

    #[error("Tensor allocation failed")]
    TensorAllocationError,

    #[error("Corrupt input tensor")]
    CorruptInputTensor,

    #[error("Model reshape failed")]
    ShapeError(#[from] ndarray::ShapeError),

    #[error("Bytes to f32 vec conversion failed")]
    BytesToVecConversionError(#[from] std::io::Error),

    #[error("Configuration of model's input type and/or shape failed")]
    CorruptInputTypeOrShape(#[from] tract_onnx::tract_core::anyhow::Error),
}
