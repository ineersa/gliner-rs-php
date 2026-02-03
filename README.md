# gliner-rs-php

**WARNING**: This project completely written by `GPT 5.2 Codex` and I am not responsible for any damages caused by it.

PHP bindings for the [`gline-rs`](https://github.com/fbilhaut/gline-rs) GLiNER library, providing fast entity extraction from PHP.

## Install / connect to PHP

1. Download the latest release archive from the GitHub releases page:

https://github.com/ineersa/gliner-rs-php/releases

Extract it to get `libgliner_rs_php.so`.
It's compiled for PHP 8.4 linux only, so you may need to compile it yourself for other platforms.

2. Copy the `.so` into your PHP extensions directory (or any path you control):

```
cp libgliner_rs_php.so /usr/local/lib/php/extensions/libgliner_rs_php.so
```

3. Enable the extension (create `gliner_rs_php.ini` in your PHP `conf.d`):

```
echo "extension=/usr/local/lib/php/extensions/libgliner_rs_php.so" > /usr/local/etc/php/conf.d/gliner_rs_php.ini
```

4. Use the extension from PHP:

```
$gliner = new GlinerWrapper($tokenizerPath, $modelPath);
$results = $gliner->predictBatch($texts, $labels);
```

## Models (required)

You must provide a GLiNER tokenizer and ONNX model file (`tokenizer.json` and `model.onnx`) on disk. 
The extension does not download models for you.

For example you can use the following models:

- Converted ONNX model (large): https://huggingface.co/ineersa/gliner-PII-onnx (converted from https://huggingface.co/nvidia/gliner-PII).
- Smaller and edge-friendly alternatives: https://huggingface.co/collections/knowledgator/gliner-pii

## IDE stubs

The file `gliner_stubs.php` provides IDE-friendly stubs for `GlinerWrapper`. Add it to your PHP project or configure it as an include path in your IDE (e.g. PhpStorm: `Settings | PHP | Include Paths`). The stubs are for autocomplete only and should not be loaded in production at runtime.

## Development

Use the Makefile targets from the project root:

```
make cargo-build
```

The compiled library will be at `target/debug/libgliner_rs_php.so`.

Run tests:

```
make cargo-test
```

Generate IDE stubs:

```
make cargo-stubs
```

## Docker test example

Build and run the included Docker test:

```
make docker-test
```

Expected output from `docker/test.php` (batch output from `predictBatch`):

```
array(3) {
  [0]=>
  array(1) {
    [0]=>
    array(6) {
      ["text"]=>
      string(10) "James Bond"
      ["label"]=>
      string(6) "person"
      ["score"]=>
      float(0.7103710174560547)
      ["start"]=>
      int(11)
      ["end"]=>
      int(21)
      ["sequence"]=>
      int(0)
    }
  }
  [1]=>
  array(1) {
    [0]=>
    array(6) {
      ["text"]=>
      string(12) "Aston Martin"
      ["label"]=>
      string(7) "vehicle"
      ["score"]=>
      float(0.9735746383666992)
      ["start"]=>
      int(11)
      ["end"]=>
      int(23)
      ["sequence"]=>
      int(1)
    }
  }
  [2]=>
  array(2) {
    [0]=>
    array(6) {
      ["text"]=>
      string(5) "Alice"
      ["label"]=>
      string(6) "person"
      ["score"]=>
      float(0.8724349737167358)
      ["start"]=>
      int(0)
      ["end"]=>
      int(5)
      ["sequence"]=>
      int(2)
    }
    [1]=>
    array(6) {
      ["text"]=>
      string(3) "Bob"
      ["label"]=>
      string(6) "person"
      ["score"]=>
      float(0.8528742790222168)
      ["start"]=>
      int(10)
      ["end"]=>
      int(13)
      ["sequence"]=>
      int(2)
    }
  }
}
```

Expected output from `docker/test.php` (single output from `predictSingle`):

```
array(3) {
  [0]=>
  array(6) {
    ["text"]=>
    string(4) "Mary"
    ["label"]=>
    string(6) "person"
    ["score"]=>
    float(0.8783774375915527)
    ["start"]=>
    int(0)
    ["end"]=>
    int(4)
    ["sequence"]=>
    int(0)
  }
  [1]=>
  array(6) {
    ["text"]=>
    string(4) "John"
    ["label"]=>
    string(6) "person"
    ["score"]=>
    float(0.8383087515830994)
    ["start"]=>
    int(9)
    ["end"]=>
    int(13)
    ["sequence"]=>
    int(0)
  }
  [2]=>
  array(6) {
    ["text"]=>
    string(6) "Berlin"
    ["label"]=>
    string(4) "city"
    ["score"]=>
    float(0.9857500791549683)
    ["start"]=>
    int(22)
    ["end"]=>
    int(28)
    ["sequence"]=>
    int(0)
  }
}
```