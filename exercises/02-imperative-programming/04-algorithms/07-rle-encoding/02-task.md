---
id          = "impp_rle"
name        = "RLE Encoding"
language    = "rust"
difficulty  = 3
description = "Implement run-length encoding and decoding of strings."
topics      = ["strings", "compression", "loops"]
---

# RLE Encoding

Run-length encoding (RLE) compresses a string by replacing runs of repeated characters with the count followed by the character.

1. `encode(input: &str) -> String`
   - compress runs of the same character into `count` + `character`
   - a run of length 1 is encoded as `"1"` + the character

2. `decode(input: &str) -> String`
   - expand `count` + `character` pairs back into the original string
   - counts may have multiple digits (e.g. `"12a"` → `"aaaaaaaaaaaa"`)

## Expected Result

```
encode("aaabbc")   → "3a2b1c"
decode("3a2b1c")   → "aaabbc"
decode("12a1b")    → "aaaaaaaaaaaab"
encode("")         → ""
```