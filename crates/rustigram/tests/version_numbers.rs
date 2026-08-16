//! The version number appears in nine places; these keep them in step.
//!
//! Every one of them is read by somebody: the install snippets are what a user
//! copies, and the internal dependency pins are what crates.io resolves against.
//! A stale pin publishes a release whose crates depend on the *previous*
//! version of each other, which builds locally and breaks for everyone else.
//!
//! Run by `cargo test`, so a version bump that misses one fails before release
//! rather than after.

#[test]
fn test_main_lib_deps() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
    version_sync::assert_contains_regex!("src/lib.rs", r#"rustigram = "{version}""#);
}

#[test]
fn test_readme_deps() {
    version_sync::assert_contains_regex!("../../README.md", r#"rustigram = "{version}""#);
}

#[test]
fn test_workspace_internal_deps_versions() {
    version_sync::assert_contains_regex!(
        "../../Cargo.toml",
        r#"(?m)^rustigram-types\s*=\s*\{\s*version\s*=\s*"{version}",\s*path\s*=\s*"crates/rustigram-types"\s*\}"#
    );
    version_sync::assert_contains_regex!(
        "../../Cargo.toml",
        r#"(?m)^rustigram-api\s*=\s*\{\s*version\s*=\s*"{version}",\s*path\s*=\s*"crates/rustigram-api"\s*\}"#
    );
    version_sync::assert_contains_regex!(
        "../../Cargo.toml",
        r#"(?m)^rustigram-bot\s*=\s*\{\s*version\s*=\s*"{version}",\s*path\s*=\s*"crates/rustigram-bot"\s*\}"#
    );
    version_sync::assert_contains_regex!(
        "../../Cargo.toml",
        r#"(?m)^rustigram-macros\s*=\s*\{\s*version\s*=\s*"{version}",\s*path\s*=\s*"crates/rustigram-macros"\s*\}"#
    );
    // rustigram-miniapp was missing here, so its pin could go stale unnoticed
    // while every other crate's was checked.
    version_sync::assert_contains_regex!(
        "../../Cargo.toml",
        r#"(?m)^rustigram-miniapp\s*=\s*\{\s*version\s*=\s*"{version}",\s*path\s*=\s*"crates/rustigram-miniapp"\s*\}"#
    );
}

/// The Mini App crate has its own README with its own install snippet.
///
/// Not covered before, so a user following it would have been told to depend on
/// whatever version was current when it was last edited by hand.
#[test]
fn test_miniapp_readme_deps() {
    version_sync::assert_contains_regex!(
        "../../crates/rustigram-miniapp/README.md",
        r#"rustigram-miniapp = "{version}""#
    );
}
