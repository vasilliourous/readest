use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    propagate_app_version();

    // Declare the app's own (non-plugin) commands in the ACL app manifest.
    // Since tauri 2.11, IPC from remote origins is always subject to ACL
    // resolution (upstream #15266); without a manifest the app commands have
    // no ACL entries at all and remote pages get "not allowed. Plugin not
    // found". The webdriver test harness serves the vitest tester page from
    // its own port, which is a remote origin, so it needs these permissions
    // granted via capabilities (see capabilities/webdriver-remote.json).
    // With a manifest defined, LOCAL windows also resolve app commands
    // through the ACL, so capabilities/default.json must grant them too.
    // Keep this list in sync with the generate_handler! list in lib.rs.
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "start_server",
            "download_file",
            "upload_file",
            "get_environment_variable",
            "get_executable_dir",
            "set_webview_info",
            "is_updater_disabled",
            "allow_paths_in_scopes",
            "read_dir",
            "parse_epub_metadata",
            "extract_epub_cover_full",
            "parse_epub_full",
            "parse_mobi_metadata",
            "extract_mobi_cover_full",
            "auth_with_safari",
            "start_apple_sign_in",
            "set_traffic_lights",
            "show_lookup_popover",
            "update_book_presence",
            "clear_book_presence",
            "clip_url",
            "spawn_fresh_browser",
            "verify_update_signature",
            "install_nightly_update",
        ]),
    ))
    .expect("failed to run tauri-build");
}

/// Bake the app version from `package.json` into the crate as `READEST_APP_VERSION`
/// (read back via `option_env!`). Sentry keys its release/environment off this
/// rather than `CARGO_PKG_VERSION`, because the crate version in `Cargo.toml` is
/// not kept in sync with the app version (and only `package.json` carries the
/// nightly `-YYYYMMDDHH` stamp). Absent/unparseable => unset, so the Rust code
/// falls back to the crate version.
fn propagate_app_version() {
    let package_json = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("package.json");
    println!("cargo:rerun-if-changed={}", package_json.display());

    if let Some(version) = read_json_string_field(&package_json, "version") {
        println!("cargo:rustc-env=READEST_APP_VERSION={version}");
    }
}

/// Read a top-level `"key": "value"` string from a JSON file without pulling in a
/// JSON parser. Returns the first match; `None` if the file/key is absent or the
/// value is empty. `package.json`'s own `"version"` is the first `"version"` key.
fn read_json_string_field(path: &Path, key: &str) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    let needle = format!("\"{key}\"");
    for line in contents.lines() {
        let Some(rest) = line.trim_start().strip_prefix(&needle) else {
            continue;
        };
        let value = rest
            .trim_start()
            .strip_prefix(':')?
            .trim()
            .trim_end_matches(',')
            .trim()
            .trim_matches('"');
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}


