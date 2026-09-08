---
id          = "strategy"
name        = "Strategy Pattern"
language    = "rust"
difficulty  = 4
description = "Swap algorithms at runtime using trait objects."
topics      = ["trait", "dyn", "strategy"]
---

# Strategy Pattern

Build a text compressor whose algorithm can be swapped at runtime.

## Your Task

1. Define a trait `CompressionStrategy` with one method: `compress(&self, data: &str) -> String`.
2. Implement two strategies:
   - `NoCompression` returns the input unchanged.
   - `RleCompression` applies run-length encoding: each run of identical characters becomes the character followed by its count (`"aaabb"` -> `"a3b2"`, `"abc"` -> `"a1b1c1"`).
3. Give `Compressor` a field `strategy: Box<dyn CompressionStrategy>`.
4. Implement `Compressor::new(strategy: Box<dyn CompressionStrategy>) -> Self` and `Compressor::compress(&self, data: &str) -> String`, which delegates to the strategy.
5. In `main`, compress the same string with each strategy and print both results.

## Expected Result

With `data = "aaabbcccc"`, the `NoCompression` compressor prints `aaabbcccc` unchanged, while the `RleCompression` compressor prints `a3b2c4`.
