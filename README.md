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

This repo no longer assumes a sibling `../XIFty` checkout, and it no longer
clones core source by default.

Prepare the canonical runtime artifact into a repo-local cache:

```bash
bash scripts/prepare-runtime.sh
```

Then run the crate:

```bash
cargo test
cargo run --example basic_usage
cargo run --example gallery_ingest
```

Runtime resolution order is:

1. bundled runtime inside the crate/package, if present
2. `XIFTY_RUNTIME_DIR`, if explicitly set
3. repo-local runtime cache from `scripts/prepare-runtime.sh`
4. `XIFTY_CORE_DIR` as an explicit source-tree override for maintainers

This keeps normal use on a runtime-artifact path while preserving an honest
source override for local maintainers.

## Status

- release-ready but still source-first
- built on the stable `xifty-ffi` ABI
- release validation now runs against canonical runtime artifacts instead of
  implicitly cloning core source
- CI validates the wrapper against canonical runtime artifacts built from the
  public XIFty core repo

## License

MIT
