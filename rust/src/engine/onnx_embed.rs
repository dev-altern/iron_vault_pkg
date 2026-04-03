//! ONNX Runtime embedding engine — behind the `onnx` feature flag.
//!
//! Load a pre-trained text embedding model (e.g., minilm-l6-v2) and
//! compute embeddings on-device. Enable with:
//! ```toml
//! iron_vault_core = { features = ["onnx"] }
//! ```

#[cfg(feature = "onnx")]
use anyhow::{anyhow, Context, Result};
#[cfg(feature = "onnx")]
use ndarray::Array2;
#[cfg(feature = "onnx")]
use ort::{Session, Value};
#[cfg(feature = "onnx")]
use std::sync::Mutex;

/// On-device embedding model using ONNX Runtime.
#[cfg(feature = "onnx")]
pub(crate) struct OnnxEmbedder {
    session: Mutex<Session>,
    dimension: usize,
}

#[cfg(feature = "onnx")]
impl OnnxEmbedder {
    /// Load an ONNX model from a file path.
    ///
    /// `dimension` is the expected output embedding dimension (e.g., 384 for minilm).
    pub(crate) fn load(model_path: &str, dimension: usize) -> Result<Self> {
        let session = Session::builder()
            .context("OnnxException: failed to create session builder")?
            .commit_from_file(model_path)
            .with_context(|| format!("OnnxException: failed to load model from {}", model_path))?;
        Ok(Self {
            session: Mutex::new(session),
            dimension,
        })
    }

    /// Compute embedding for a single text.
    ///
    /// This is a simplified tokenizer that works with sentence-transformer models
    /// that accept raw text input. For production, use a proper tokenizer.
    pub(crate) fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let session = self.session.lock().unwrap();

        // Simple tokenization: convert chars to token IDs
        // This is a placeholder — real models need a WordPiece/BPE tokenizer
        let token_ids: Vec<i64> = text
            .chars()
            .take(512)
            .map(|c| c as i64)
            .collect();
        let attention_mask: Vec<i64> = vec![1i64; token_ids.len()];
        let seq_len = token_ids.len();

        let input_ids = Array2::from_shape_vec((1, seq_len), token_ids)
            .context("OnnxException: failed to create input tensor")?;
        let attn_mask = Array2::from_shape_vec((1, seq_len), attention_mask)
            .context("OnnxException: failed to create attention mask")?;

        let outputs = session.run(ort::inputs![input_ids, attn_mask]?)
            .context("OnnxException: inference failed")?;

        // Extract embedding from output (typically shape [1, seq_len, dim] or [1, dim])
        let output = &outputs[0];
        let tensor = output.try_extract_tensor::<f32>()
            .context("OnnxException: failed to extract output tensor")?;

        // Mean pooling over sequence dimension
        let shape = tensor.shape();
        let embedding = if shape.len() == 3 {
            // [batch, seq_len, dim] → mean over seq_len
            let dim = shape[2];
            let mut pooled = vec![0.0f32; dim];
            let seq = shape[1];
            for s in 0..seq {
                for d in 0..dim {
                    pooled[d] += tensor[[0, s, d]];
                }
            }
            for v in pooled.iter_mut() {
                *v /= seq as f32;
            }
            pooled
        } else if shape.len() == 2 {
            // [batch, dim]
            tensor.row(0).to_vec()
        } else {
            return Err(anyhow!("OnnxException: unexpected output shape {:?}", shape));
        };

        // Normalize to unit length
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let normalized: Vec<f32> = if norm > 0.0 {
            embedding.iter().map(|x| x / norm).collect()
        } else {
            embedding
        };

        Ok(normalized)
    }

    /// Get the embedding dimension.
    pub(crate) fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Stub when onnx feature is not enabled.
#[cfg(not(feature = "onnx"))]
pub(crate) struct OnnxEmbedder;

#[cfg(not(feature = "onnx"))]
impl OnnxEmbedder {
    pub(crate) fn load(_model_path: &str, _dimension: usize) -> anyhow::Result<Self> {
        Err(anyhow::anyhow!(
            "OnnxException: ONNX support not enabled. \
             Rebuild with `features = [\"onnx\"]` in Cargo.toml"
        ))
    }
}
