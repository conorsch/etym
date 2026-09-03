//! End-to-end CLI tests. Each test runs the `etym` binary against a word and
//! inspects the output. Cargo sets the `CARGO_BIN_EXE_etym` env var so we don't
//! need to guess the binary path.
//!
//! Tests that hit the live network are gated behind the `integration` feature
//! so that `cargo test` works offline. Run them with:
//!
//!     just integration

use std::path::PathBuf;
use std::process::{Command, Output};

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_etym"))
}

fn run(args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .output()
        .expect("failed to run etym")
}

fn stderr(out: &Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("stderr is utf-8")
}

#[test]
fn missing_word_returns_error() {
    let result = run(&[]);
    assert!(!result.status.success(), "should fail without word");
    assert!(!stderr(&result).is_empty());
}

#[cfg(feature = "integration")]
mod live_network {
    use super::run;
    use std::process::Output;

    fn stdout(out: &Output) -> String {
        String::from_utf8(out.stdout.clone()).expect("stdout is utf-8")
    }

    #[test]
    fn viking_lookup() {
        let result = run(&["viking"]);
        assert!(result.status.success(), "lookup failed: {result:?}");
        let out = stdout(&result);
        assert!(out.contains("Viking"));
        assert!(out.contains("Scandinavian pirate"));
        assert!(out.contains("vikingr"));
    }

    #[test]
    fn scrimshaw_lookup() {
        let result = run(&["scrimshaw"]);
        assert!(result.status.success(), "lookup failed: {result:?}");
        let out = stdout(&result);
        assert!(out.contains("scrimshaw"));
        assert!(out.contains("shell or piece of ivory"));
    }
}
