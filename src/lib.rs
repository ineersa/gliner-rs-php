#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use gliner::model::input::text::TextInput;
use gliner::model::params::Parameters;
use gliner::model::pipeline::span::SpanMode;
use gliner::model::GLiNER;
use gliner::text::span::Span;
use orp::params::RuntimeParameters;

#[php_class]
pub struct GlinerWrapper {
    model: GLiNER<SpanMode>,
}

#[php_impl]
impl GlinerWrapper {
    /// Load the model and tokenizer from the file system.
    ///
    /// @param string $tokenizer_path Path to tokenizer.json
    /// @param string $model_path Path to model.onnx
    /// @throws Exception If model loading fails.
    pub fn __construct(tokenizer_path: String, model_path: String) -> PhpResult<Self> {
        let model = GLiNER::<SpanMode>::new(
            Parameters::default(),
            RuntimeParameters::default(),
            &tokenizer_path,
            &model_path,
        )
        .map_err(|error: Box<dyn std::error::Error + Send + Sync>| {
            PhpException::default(error.to_string())
        })?;

        Ok(Self { model })
    }

    /// Perform prediction on a single text input.
    ///
    /// @param string $text Text to process
    /// @param string[] $labels Entity labels to look for
    /// @return array<int, array{text: string, label: string, score: float, start: int, end: int, sequence: int}> Array of extracted entities
    pub fn predict_single(&self, text: String, labels: Vec<String>) -> PhpResult<ZBox<ZendHashTable>> {
        let input = build_text_input(vec![text], labels)?;
        let predictions = self
            .model
            .inference(input)
            .map_err(|error| PhpException::default(error.to_string()))?;
        let spans = predictions
            .spans
            .get(0)
            .ok_or_else(|| PhpException::default("No predictions returned".into()))?;
        spans_to_php_sequence(spans)
    }

    /// Perform batch prediction.
    ///
    /// @param string[] $texts Array of texts to process
    /// @param string[] $labels Array of entity labels to look for
    /// @return array<int, array<int, array{text: string, label: string, score: float, start: int, end: int, sequence: int}>> Array of extracted entities per text
    pub fn predict_batch(
        &self,
        texts: Vec<String>,
        labels: Vec<String>,
    ) -> PhpResult<ZBox<ZendHashTable>> {
        let input = build_text_input(texts, labels)?;
        let predictions = self
            .model
            .inference(input)
            .map_err(|error| PhpException::default(error.to_string()))?;

        spans_to_php_batch(&predictions.spans)
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<GlinerWrapper>()
}

pub fn build_text_input(texts: Vec<String>, labels: Vec<String>) -> PhpResult<TextInput> {
    TextInput::new(texts, labels).map_err(|error| PhpException::default(error.to_string()))
}

fn spans_to_php_batch(spans: &[Vec<Span>]) -> PhpResult<ZBox<ZendHashTable>> {
    let mut batch = ZendHashTable::new();
    for sequence_spans in spans {
        batch
            .push(spans_to_php_sequence(sequence_spans)?)
            .map_err(|error| PhpException::default(error.to_string()))?;
    }
    Ok(batch)
}

fn spans_to_php_sequence(spans: &[Span]) -> PhpResult<ZBox<ZendHashTable>> {
    let mut sequence = ZendHashTable::new();
    for span in spans {
        sequence
            .push(span_to_php(span)?)
            .map_err(|error| PhpException::default(error.to_string()))?;
    }
    Ok(sequence)
}

fn span_to_php(span: &Span) -> PhpResult<ZBox<ZendHashTable>> {
    let mut entry = ZendHashTable::new();
    let (start, end) = span.offsets();

    entry
        .insert("text", span.text())
        .map_err(|error| PhpException::default(error.to_string()))?;
    entry
        .insert("label", span.class())
        .map_err(|error| PhpException::default(error.to_string()))?;
    entry
        .insert("score", span.probability())
        .map_err(|error| PhpException::default(error.to_string()))?;
    entry
        .insert("start", start as i64)
        .map_err(|error| PhpException::default(error.to_string()))?;
    entry
        .insert("end", end as i64)
        .map_err(|error| PhpException::default(error.to_string()))?;
    entry
        .insert("sequence", span.sequence() as i64)
        .map_err(|error| PhpException::default(error.to_string()))?;

    Ok(entry)
}