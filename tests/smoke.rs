//! ASCII source gate + scalar smoke tests.

use std::fs;
use std::path::PathBuf;

use rusty_symbols::{list, math, nav, status, structure, VS15};

fn src_rs_files() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read src") {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

#[test]
fn source_is_pure_ascii() {
    let mut offenders = Vec::new();
    for path in src_rs_files() {
        let bytes = fs::read(&path).expect("read");
        for (i, b) in bytes.iter().enumerate() {
            if *b > 0x7F {
                offenders.push(format!(
                    "{}: offset {i} has non-ASCII byte 0x{b:02X}",
                    path.display()
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "rusty_symbols source must stay ASCII (use \\u{{...}} escapes):\n{}",
        offenders.join("\n")
    );
}

#[test]
fn status_scalars() {
    assert_eq!(status::OK, concat!("\u{2713}", "\u{FE0E}"));
    assert_eq!(status::FAIL, "\u{2717}");
    assert_eq!(status::LIVE, "\u{25CF}");
    assert_eq!(status::PLAY, concat!("\u{25B6}", "\u{FE0E}"));
}

#[test]
fn nav_and_vs15() {
    assert_eq!(nav::RIGHT, "\u{2192}");
    assert_eq!(nav::COLLAPSE, concat!("\u{25B2}", "\u{FE0E}"));
    assert_eq!(VS15, "\u{FE0E}");
}

#[test]
fn structure_math_list() {
    assert!(!structure::RULE_H.is_empty());
    assert!(!math::GTE.is_empty());
    assert!(!list::BULLET.is_empty());
}

#[cfg(feature = "rusty-alloc")]
#[test]
fn rusty_alloc_on_by_default() {
    assert!(rusty_symbols::rusty_alloc_enabled());
}

#[cfg(not(feature = "rusty-alloc"))]
#[test]
fn rusty_alloc_off_when_opted_out() {
    assert!(!rusty_symbols::rusty_alloc_enabled());
}
