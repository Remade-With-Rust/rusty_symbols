#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Unicode glyph constants for Rust UIs -- ASCII `\u{...}` escapes, semantically grouped.
//!
//! Sibling of [`rusty_tokens`](https://crates.io/crates/rusty_tokens) and
//! [`rusty_a11y`](https://crates.io/crates/rusty_a11y). By default installs
//! [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc) via
//! [`rusty_alloc_default`](https://crates.io/crates/rusty_alloc_default)
//! (opt out with `default-features = false`).

/// Whether this build pulled in the default `rusty_alloc` install.
pub const fn rusty_alloc_enabled() -> bool {
    cfg!(feature = "rusty-alloc")
}

/// Whether the hardened `secure` allocator profile is compiled in.
pub const fn secure_allocator_enabled() -> bool {
    cfg!(feature = "secure")
}

pub mod list;
pub mod math;
pub mod nav;
pub mod status;
pub mod structure;

/// Variation selector-15: force text presentation (not emoji colour).
pub const VS15: &str = "\u{FE0E}";

/// Variation selector-16: force emoji presentation.
pub const VS16: &str = "\u{FE0F}";
