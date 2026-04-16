# XIFtyRust

Rust crate for XIFty.

This crate currently links against the XIFty core repository through the stable
`xifty-ffi` C ABI. Local development expects a sibling checkout of the core repo
at:

- `../XIFty`

You can override that path with `XIFTY_CORE_DIR`.

## Local Development

```bash
cargo test
cargo run --example basic_usage
```

