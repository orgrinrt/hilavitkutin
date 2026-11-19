// A consumer's build script, kept where the consumer chose. What follows
// `main` stands in for what one call into `hilavitkutin-build` would do
// in the script's own std context: find the workspace, put the wrapper
// and its config line there when they are missing, and hand the crate's
// flags over.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const WRAPPER_NAME: &str = "hilavitkutin-rustc-wrapper";
const WRAPPER_TEXT: &str = "#!/bin/sh\n\
    # Written by hilavitkutin-build: appends the flags a crate's build script chose for it.\n\
    set -f\n\
    exec \"$@\" $HILAVITKUTIN_BUILD_RUSTFLAGS\n";
const CONFIG_TEXT: &str =
    "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n";

fn main() {
    println!("cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER");
    println!("cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=--cfg sketch_flag_arrived");

    let wrapped = std::env::var("RUSTC_WORKSPACE_WRAPPER")
        .map(|path| path.ends_with(WRAPPER_NAME))
        .unwrap_or(false);
    if wrapped {
        return;
    }
    match workspace_root() {
        Some(root) if under_cargo_home(&root) => {
            println!(
                "cargo::warning=a dependency's checkout, nothing written: {}",
                root.display()
            );
        },
        Some(root) => {
            match wire(&root) {
                Ok(wrote) => {
                    println!(
                        "cargo::warning=hilavitkutin-build {wrote} under {}/.cargo; this build ran without the wrapper, the next one takes the flags",
                        root.display()
                    )
                },
                Err(why) => {
                    println!("cargo::warning=hilavitkutin-build could not write the wrapper: {why}")
                },
            }
        },
        None => println!("cargo::warning=hilavitkutin-build found no workspace root"),
    }
}

/// The root of the workspace this package is a member of, asked of cargo.
fn workspace_root() -> Option<PathBuf> {
    let cargo = std::env::var_os("CARGO")?;
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")?;
    let out = Command::new(cargo)
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .current_dir(manifest_dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let manifest = PathBuf::from(String::from_utf8(out.stdout).ok()?.trim());
    manifest.parent().map(Path::to_path_buf)
}

/// A checkout cargo fetched is no tree of the consumer's to write into.
fn under_cargo_home(root: &Path) -> bool {
    std::env::var_os("CARGO_HOME").is_some_and(|home| root.starts_with(home))
}

/// Write the wrapper and the config line where they are missing.
fn wire(root: &Path) -> Result<&'static str, String> {
    let dir = root.join(".cargo");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let wrapper = dir.join(WRAPPER_NAME);
    if fs::read_to_string(&wrapper).ok().as_deref() != Some(WRAPPER_TEXT) {
        fs::write(&wrapper, WRAPPER_TEXT).map_err(|e| e.to_string())?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755))
            .map_err(|e| e.to_string())?;
    }
    let config = dir.join("config.toml");
    match fs::read_to_string(&config) {
        Ok(text) if text.contains("rustc-workspace-wrapper") => Ok("wrote the wrapper"),
        Ok(_) => {
            Err(format!(
                "{} exists and names no wrapper; add `build.rustc-workspace-wrapper = \".cargo/{WRAPPER_NAME}\"` to it",
                config.display()
            ))
        },
        Err(_) => {
            fs::write(&config, CONFIG_TEXT).map_err(|e| e.to_string())?;
            Ok("wrote the wrapper and its config line")
        },
    }
}
