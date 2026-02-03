<?php

// Stubs for gliner-rs-php

namespace {
    class GlinerWrapper {
        /**
         * Perform prediction on a single text input.
         *
         * @param string $text Text to process
         * @param string[] $labels Entity labels to look for
         * @return array<int, array{text: string, label: string, score: float, start: int, end: int, sequence: int}> Array of extracted entities
         */
        public function predictSingle(string $text, array $labels): array {}

        /**
         * Perform batch prediction.
         *
         * @param string[] $texts Array of texts to process
         * @param string[] $labels Array of entity labels to look for
         * @return array<int, array<int, array{text: string, label: string, score: float, start: int, end: int, sequence: int}>> Array of extracted entities per text
         */
        public function predictBatch(array $texts, array $labels): array {}

        /**
         * Load the model and tokenizer from the file system.
         *
         * @param string $tokenizer_path Path to tokenizer.json
         * @param string $model_path Path to model.onnx
         * @throws Exception If model loading fails.
         */
        public function __construct(string $tokenizer_path, string $model_path) {}
    }
}
