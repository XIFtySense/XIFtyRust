# XIFty for Rust

`XIFtyRust` is the official Rust binding repo for XIFty.

It provides a safe Rust wrapper over the stable `xifty-ffi` ABI so Rust
applications can probe files and extract XIFty metadata views without binding
directly to the C surface.

## What It Does

XIFty exposes four complementary metadata views:

- `raw`
- `interpreted`
- `normalized`
- `report`

This crate keeps that contract intact and adds Rust-native error handling.

## Quick Example

```rust
let output = xifty_rust::extract("photo.jpg", xifty_rust::ViewMode::Normalized)?;
let fields = output["normalized"]["fields"].as_array().unwrap();
```

## API

- `version()`
- `probe(path)`
- `extract(path, view)`

## Why Use It

Use this crate when you want:

- native Rust access to XIFty
- normalized fields for application logic
- raw and interpreted metadata for provenance-sensitive workflows
- a thin safe wrapper around the stable C ABI

## Local Setup

This repo no longer assumes a sibling `../XIFty` checkout.

Prepare the core dependency into a repo-local cache:

```bash
bash scripts/prepare-core.sh
```

Then run the crate:

```bash
cargo test
cargo run --example basic_usage
cargo run --example gallery_ingest
```

You can still override the core location explicitly with `XIFTY_CORE_DIR`.

## Status

- source-first and usable today
- built on the stable `xifty-ffi` ABI
- CI validates the wrapper against the public XIFty core repo

## License

MIT
