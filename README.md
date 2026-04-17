# XIFtyRust

Rust binding crate for [XIFty](https://github.com/XIFtySense/XIFty).

`XIFtyRust` is a safe Rust wrapper over the stable `xifty-ffi` C ABI. It is
ready for source-based use today and is intended to become the canonical Rust
crate for consumers who want the XIFty engine without binding directly to the C
surface.

## What You Get

- `version()` for the bound core version
- `probe(path)` for fast format detection
- `extract(path, view)` for the standard JSON views
- Rust-native error handling around the ABI boundary

## Quickstart

Clone the public core repo as a sibling checkout, then run the crate against it:

```bash
git clone git@github.com:XIFtySense/XIFty.git ../XIFty
cargo test
cargo run --example basic_usage
```

If your core checkout lives elsewhere, set `XIFTY_CORE_DIR`:

```bash
XIFTY_CORE_DIR=/path/to/XIFty cargo test
```

## Status

- source-first and usable today
- built on the stable `xifty-ffi` ABI
- CI validates the wrapper against the public XIFty core repo on every push
- crate metadata is in place for future crates.io distribution

## License

MIT
