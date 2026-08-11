# rusty_symbols

[![Remade With Rust](https://img.shields.io/badge/Remade%20With-Rust-000?logo=rust&logoColor=fff)](https://github.com/remade-with-rust)
[![By Mata Network](https://img.shields.io/badge/by-Mata%20Network-5b2be0)](https://www.mata.network)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE-MIT)
![Platforms: Windows · macOS · Linux · Web · WASM](https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux%20%C2%B7%20Web%20%C2%B7%20WASM-informational)
![MSRV: 1.73](https://img.shields.io/badge/MSRV-1.73-informational)

> **rusty_symbols** is an open-source Unicode glyph toolkit for any Rust UI --
> **semantically named constants + VS15 presentation pinning** -- pure Rust.
> Application `.rs` files stay ASCII; glyphs never scatter as raw literals that
> Windows-1252 round-trips can mojibake. By default installs
> [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc) (opt out below).
> Sibling of [`rusty_tokens`](https://crates.io/crates/rusty_tokens) and
> [`rusty_a11y`](https://crates.io/crates/rusty_a11y).

> **Status -- v0.1.0.** Pin `rusty_symbols = "0.1"` from crates.io
> (or git tag `v0.1.0`). Core is `no_std` / wasm-checked.

---

## The headline

> **One immune source of truth.** Glyphs live once, as ASCII source.

| Dimension | Scattered literals | **rusty_symbols** | Goal |
|---|:---:|:---:|:---:|
| Mojibake-proof source | every file is a site | **ASCII `\u{...}` escapes** | structural |
| Naming | raw codepoints | **semantic modules** | maintain |
| Presentation (WebView) | platform-dependent | **VS15 pinned glyphs** | uniform |
| Allocator | system / C | **`rusty_alloc` by default** | opt-out |
| Dependencies | -- | **none when opted out** | maintain |
| License | mixed | **MIT** | -- |

---

## Install

```toml
rusty_symbols = "0.1"
# bring your own allocator:
# rusty_symbols = { version = "0.1", default-features = false }
# hardened allocator:
# rusty_symbols = { version = "0.1", features = ["secure"] }
```

| Feature | Default | Provides |
|---------|---------|----------|
| `rusty-alloc` | **yes** | [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc) via [`rusty_alloc_default`](https://crates.io/crates/rusty_alloc_default) |
| `secure` | no | enables `rusty-alloc` + guard pages / encrypted free lists |

Combining with `rusty_tokens` / `rusty_a11y` defaults is safe -- all three share
one `rusty_alloc_default` link. Libraries and `mata-alloc` apps must set
`default-features = false`.

MSRV: **1.73**.

## Quick start

```rust
use rusty_symbols::{nav, status};

fn label_ok() -> String {
    format!("{} ready", status::OK)
}
```

```sh
cargo test
cargo test --no-default-features
```

## Features

- **status** -- ok / fail / warn / timer / alarm / live / play / stop.
- **nav** -- arrows, hooks, branch, collapse (VS15 on triangles).
- **structure** -- horizontal rule, tree tee / corner.
- **math** -- gte / lte / approx / times.
- **list** -- bullet / middot.
- **rusty_alloc** -- on by default; `secure` for hardening.

## Platform support

| Platform | Status |
|---|---|
| Windows | yes |
| macOS | yes |
| Linux | yes |
| Web (Dioxus / browsers) | yes |
| WASM (`wasm32-unknown-unknown`) | yes (`no_std`; default `rusty_alloc` covers the heap) |

## Remade With Rust

**Remade With Rust** ([Mata Network](https://www.mata.network)) rebuilds essential
tooling in Rust -- memory safety, predictable performance, permissive license.

-> **[github.com/remade-with-rust](https://github.com/remade-with-rust)**

Family: **rusty_symbols** ·
[rusty_tokens](https://github.com/Remade-With-Rust/rusty_tokens) ·
[rusty_a11y](https://github.com/Remade-With-Rust/rusty_a11y)

## License

MIT -- [LICENSE-MIT](LICENSE-MIT).

## Trademark

"Remade With Rust", "Mata", and "Mata Network" are marks of Mata Network.
