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
}
