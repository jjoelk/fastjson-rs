# fastjson-rs

Blazing fast JSON parser with zero-copy deserialization for Rust.

## Features

- Zero-copy string deserialization
- Lightweight `FastValue` enum for dynamic JSON
- Serde-compatible

## Usage

```rust
use fastjson_rs::FastValue;

let value = FastValue::parse(r#"{"name": "example"}"#).unwrap();
println!("{:?}", value);
```

## Install

```toml
[dependencies]
fastjson-rs = "0.2.1"
```

## License

MIT
