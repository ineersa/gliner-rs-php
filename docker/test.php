<?php

$gliner = new GlinerWrapper(
    __DIR__ . '/models/tokenizer.json',
    __DIR__ . '/models/model.onnx'
);

$texts = [
    "My name is James Bond.",
    "I drive an Aston Martin.",
    "Alice met Bob in Paris before flying to New York."
];
$labels = ["person", "vehicle"]; 

$results = $gliner->predictBatch($texts, $labels);
var_dump($results);

$single_text = "Mary and John visited Berlin.";
$single_labels = ["person", "city"]; 

$single_result = $gliner->predictSingle($single_text, $single_labels);
var_dump($single_result);