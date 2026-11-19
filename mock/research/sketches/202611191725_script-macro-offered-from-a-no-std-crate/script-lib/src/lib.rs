//! Stands in for `hilavitkutin-build`: `no_std`, no allocation, and it
//! still offers the one line a build script writes. The decisions and
//! the text are functions here; what touches the host is the macro's
//! expansion, which is the script's own code.

#![no_std]

use core::fmt;

/// The wrapper's file name under the workspace's `.cargo/`.
pub const WRAPPER_NAME: &str = "hilavitkutin-rustc-wrapper";

/// The prototypes a script may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prototype {
    Default,
    Deterministic,
}

/// What the script does about the wiring, decided from what it read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wiring {
    /// The wrapper is already named: nothing to write.
    Wired,
    /// The root is a checkout cargo fetched: nothing is written.
    Fetched,
    /// The workspace is the consumer's own: write what is missing.
    Write,
}

/// Decide from the value of `RUSTC_WORKSPACE_WRAPPER` and whether the
/// workspace root sits under `CARGO_HOME`.
pub fn wiring(wrapper: Option<&str>, root_under_cargo_home: bool) -> Wiring {
    match wrapper {
        Some(path) if path.ends_with(WRAPPER_NAME) => Wiring::Wired,
        _ if root_under_cargo_home => Wiring::Fetched,
        _ => Wiring::Write,
    }
}

/// Whether a config file's text already names the wrapper.
pub fn config_names_wrapper(text: &str) -> bool {
    text.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with("rustc-workspace-wrapper") && line.contains(WRAPPER_NAME)
    })
}

/// The wrapper's text.
pub fn write_wrapper<W: fmt::Write>(out: &mut W) -> fmt::Result {
    out.write_str("#!/bin/sh\nset -f\nexec \"$@\" $HILAVITKUTIN_BUILD_RUSTFLAGS\n")
}

/// The config file written where none exists.
pub fn write_config<W: fmt::Write>(out: &mut W) -> fmt::Result {
    write!(
        out,
        "[build]\nrustc-workspace-wrapper = \".cargo/{WRAPPER_NAME}\"\n"
    )
}

/// The directives of one prototype.
pub fn configure<W: fmt::Write>(prototype: Prototype, out: &mut W) -> fmt::Result {
    out.write_str("cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER\n")?;
    out.write_str("cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=--cfg sketch_flag_arrived")?;
    if prototype == Prototype::Deterministic {
        out.write_str(" --cfg sketch_deterministic")?;
    }
    out.write_str("\n")
}

/// The one line of a build script. The expansion is std code in the
/// script; this crate names `::std` nowhere outside it.
#[macro_export]
macro_rules! script {
    ($prototype:ident) => {{
        let mut out = ::std::string::String::new();
        $crate::configure($crate::Prototype::$prototype, &mut out)
            .expect("a String takes every write");
        ::std::print!("{out}");

        let root = ::std::env::var_os("CARGO").and_then(|cargo| {
            let found = ::std::process::Command::new(cargo)
                .args(["locate-project", "--workspace", "--message-format", "plain"])
                .current_dir(::std::env::var_os("CARGO_MANIFEST_DIR")?)
                .output()
                .ok()?;
            let manifest = ::std::string::String::from_utf8(found.stdout).ok()?;
            let manifest = ::std::path::PathBuf::from(manifest.trim());
            manifest.parent().map(::std::path::Path::to_path_buf)
        });
        if let Some(root) = root {
            // A tree cargo unpacked carries `Cargo.toml.orig` at its root:
            // a registry source, and the copy `cargo package` verifies.
            let fetched = ::std::env::var_os("CARGO_HOME").is_some_and(|home| root.starts_with(home))
                || root.join("Cargo.toml.orig").exists();
            let wrapper = ::std::env::var("RUSTC_WORKSPACE_WRAPPER").ok();
            match $crate::wiring(wrapper.as_deref(), fetched) {
                $crate::Wiring::Wired => {},
                $crate::Wiring::Fetched => {
                    ::std::println!("cargo::warning=sketch: fetched checkout, nothing written: {}", root.display());
                },
                $crate::Wiring::Write => {
                    let dir = root.join(".cargo");
                    let mut text = ::std::string::String::new();
                    $crate::write_wrapper(&mut text).expect("a String takes every write");
                    let wrapper = dir.join($crate::WRAPPER_NAME);
                    let config = dir.join("config.toml");
                    let said = (|| -> ::std::io::Result<&'static str> {
                        ::std::fs::create_dir_all(&dir)?;
                        if ::std::fs::read_to_string(&wrapper).ok().as_deref() != Some(text.as_str()) {
                            ::std::fs::write(&wrapper, &text)?;
                        }
                        #[cfg(unix)]
                        {
                            use ::std::os::unix::fs::PermissionsExt as _;
                            ::std::fs::set_permissions(&wrapper, ::std::fs::Permissions::from_mode(0o755))?;
                        }
                        match ::std::fs::read_to_string(&config) {
                            Ok(have) if $crate::config_names_wrapper(&have) => Ok("wrote the wrapper"),
                            Ok(_) => Ok("wrote the wrapper; config.toml exists and names no wrapper, add `build.rustc-workspace-wrapper` to it"),
                            Err(_) => {
                                let mut line = ::std::string::String::new();
                                $crate::write_config(&mut line).expect("a String takes every write");
                                ::std::fs::write(&config, line)?;
                                Ok("wrote the wrapper and its config line")
                            },
                        }
                    })();
                    match said {
                        Ok(said) => ::std::println!("cargo::warning=sketch: {said}, under {}", dir.display()),
                        Err(why) => ::std::println!("cargo::warning=sketch: could not write the wiring: {why}"),
                    }
                },
            }
        }
    }};
}
