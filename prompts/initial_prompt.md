Here is a detailed task description and implementation plan for generating PHP bindings for `gliner-rs`, utilizing `ext-php-rs` to ensure load efficiency and state persistence.

# Task Description: High-Performance PHP Bindings for Gliner-rs

## 1. Objective
Create a native PHP extension using Rust that wraps the `gliner-rs` library. The primary goal is **efficiency**: the GLiNER model must be loaded into memory **once** (via a PHP Class instantiation) and reused for multiple prediction calls within the same request lifecycle. The extension must support both single-text and batch-text predictions.

## 2. Prerequisites & Environment
Ensure the development environment meets the following requirements based on the documentation:
*   **PHP:** Version 8.1 or later is required.
*   **Rust:** Modern stable Rust (approx 1.57+), though Nightly is required if compiling for Windows.
*   **Clang:** Version 5.0 or later (required by `ext-php-rs`).
*   **Tooling:** Install `cargo-php` for building and stub generation:
    ```bash
    cargo install cargo-php --locked
    ```


## 3. Implementation Plan

### Step 1: Project Initialization & Dependencies
Initialize a new Rust library and configure `Cargo.toml` to include `gliner-rs` and `ext-php-rs`.

**`Cargo.toml` configuration:**
*   **Crate Type:** Must be `cdylib` to be loaded as a PHP extension.
*   **Dependencies:**
    *   `ext-php-rs`: Enable the `anyhow` feature to map Rust errors to PHP exceptions.
    *   `gliner-rs`: Version `1.0` or higher (to ensure `ort` pinning stability).
    *   `ort`: Be aware of pinning specific versions if using older `gliner-rs` versions, though 1.0.0+ resolves this.

### Step 2: Create the `Gliner` PHP Class in Rust
To satisfy the "load model once" requirement, we must use the `#[php_class]` macro. This allows us to store the loaded `gliner-rs::GLiNER` struct inside a Rust struct that maps to a PHP object.

**Structure Design:**
*   Define a struct `GlinerModel` containing the loaded model.
*   Implement `__construct` to accept paths to the `model.onnx` and `tokenizer.json` files.
*   **Why:** This prevents reloading the ONNX model from disk on every `predict` call, which is critical for performance.

### Step 3: Implement Inference Methods
We need to map PHP arrays to Rust types (`Vec<String>`) to interface with `gliner-rs`.

**Required Methods:**
1.  **Constructor:** `new(string $model_path, string $tokenizer_path)`
    *   Initializes `GLiNER::new` using the provided paths.
    *   Should handle `Parameters` and `RuntimeParameters` (defaults are usually sufficient).
2.  **`predict_single(string $text, array $labels): array`**
    *   Wraps `TextInput::from_str` with a single string.
    *   Calls `model.inference()`.
3.  **`predict_batch(array $texts, array $labels): array`**
    *   Wraps `TextInput::from_str` passing the slice of strings directly. `gliner-rs` natively supports batch inputs (`&["text1", "text2"]`).

### Step 4: Data Conversion (Rust <-> PHP)
The `gliner-rs` inference returns complex Rust structs. These must be converted into PHP Arrays or Objects (Zvals).

*   **Input:** `ext-php-rs` automatically handles conversion from PHP `array` to Rust `Vec<String>`.
*   **Output:** You will likely need to iterate over the prediction results and build a PHP associative array returning keys like `text`, `label`, `score`, and `start`/`end` indices.

## 4. Proposed Code Structure (Draft)

Below is a draft of how the Rust code should look based on the `ext-php-rs` and `gliner-rs` APIs.

```rust
use ext_php_rs::prelude::*;
use gliner_rs::{GLiNER, Parameters, RuntimeParameters, TextInput};
use std::path::PathBuf;

#[php_class]
pub struct GlinerWrapper {
    // The internal model. Wrapped in Option because it's initialized in __construct
    inner: Option<GLiNER>,
}

#[php_impl]
impl GlinerWrapper {
    /// Load the model and tokenizer from the file system.
    /// @param string $tokenizer_path Path to tokenizer.json
    /// @param string $model_path Path to model.onnx
    /// @throws Exception If model loading fails.
    pub fn __construct(tokenizer_path: String, model_path: String) -> PhpResult<Self> {
        // Initialize GLiNER
        // Note: Using defaults for Parameters as per documentation
        let model = GLiNER::new(
            Parameters::default(),
            RuntimeParameters::default(),
            &tokenizer_path,
            &model_path,
        ).map_err(|e| PhpException::default(e.to_string()))?;

        Ok(Self { inner: Some(model) })
    }

    /// Perform batch prediction.
    /// @param string[] $texts Array of texts to process
    /// @param string[] $labels Array of entity labels to look for
    /// @return array[] Array of extracted entities
    pub fn predict_batch(&self, texts: Vec<String>, labels: Vec<String>) -> PhpResult<Vec<Vec<Vec<String>>>> {
        let model = self.inner.as_ref().ok_or(PhpException::default("Model not initialized".into()))?;

        // Convert inputs to format required by TextInput::from_str
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

        // Prepare Input
        let input = TextInput::from_str(&text_refs, &label_refs)
            .map_err(|e| PhpException::default(e.to_string()))?;

        // Run Inference
        let predictions = model.inference(input)
            .map_err(|e| PhpException::default(e.to_string()))?;

        // TODO: Map the 'predictions' Rust struct into a PHP-friendly array format.
        // This part requires mapping the Entity struct fields (text, label, score) 
        // to a generic PHP array structure.
        
        Ok(vec![]) // Placeholder return
    }
}

/// Helper to register the extension module
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
```

## 5. Build & Deployment Instructions

1.  **Build the Extension:**
    Run the release build to generate the `.so` (Linux) or `.dll` (Windows) file.
    ```bash
    cargo php install --release
    ```


2.  **Generate Stubs (IDE Support):**
    Generate a PHP file for IDE autocompletion.
    ```bash
    cargo php stubs --stdout > gliner_stubs.php
    ```


3.  **Runtime Usage (PHP):**
    ```php
    // Load the model once
    $gliner = new GlinerWrapper(
        "models/tokenizer.json", 
        "models/model.onnx"
    );

    // Batch Predict
    $texts = ["My name is James Bond", "I drive an Aston Martin"];
    $labels = ["person", "vehicle"];
    
    $results = $gliner->predict_batch($texts, $labels);
    var_dump($results);
    ```

## 6. Documentation & Resources

### Key Documentation Links
*   **ext-php-rs Guide:** [https://ext-php.rs](https://ext-php.rs)
    *   *Why useful:* Covers macros like `#[php_class]` and type conversions.
*   **ext-php-rs Docs.rs:** [https://docs.rs/ext-php-rs](https://docs.rs/ext-php-rs)
    *   *Why useful:* Technical API reference for `IntoZval` and `FromZval` (converting data types).
*   **Gline-rs Repository:** [https://github.com/fbilhaut/gline-rs](https://github.com/fbilhaut/gline-rs)
    *   *Why useful:* Contains the inference API usage and model downloading instructions.
*   **Gline-rs Models:** Hugging Face links for ONNX models (Span and Token modes) are listed in the README.

### Known Issues / Notes
*   **Windows Builds:** If building on Windows, you must use Rust Nightly due to the "vectorcall" convention.
*   **ORT Versioning:** If you encounter build errors regarding `ort`, ensure you are using `gliner-rs` v1.0.0+, which pins the transitive dependency explicitly.
*   **GPU Support:** To enable GPU support (CUDA), you must enable the `cuda` feature flag in `gline-rs` via your `Cargo.toml` and configure `RuntimeParameters` with execution providers.