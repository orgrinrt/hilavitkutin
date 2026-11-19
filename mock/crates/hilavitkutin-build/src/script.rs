//! The one line of a build script.
//!
//! The crate is `no_std` and touches neither the environment nor the
//! filesystem, and a build script has to do both. `script!` is where the
//! two meet: its expansion is the script's own code, compiled in the
//! script and never here, and every decision and every text in it comes
//! from a function of this crate.

/// Run a [`Prototype`](crate::Prototype) as the build script's body.
///
/// ```no_run
/// fn main() {
///     hilavitkutin_build::script!(Default);
/// }
/// ```
///
/// The expansion prints the prototype's directives, asks cargo for the
/// workspace root (`cargo locate-project --workspace`, from
/// `CARGO_MANIFEST_DIR`), reads the wrapper and the `config.toml` under
/// its `.cargo/`, and does what [`plan`](crate::plan) answers: it writes
/// the wrapper where the file is missing or holds another text, writes
/// the config file where there is none, prints the notice, and asks to be
/// run again when either file changes. A root under `CARGO_HOME`, one
/// that holds `Cargo.toml.orig`, and one other than the root the wrapper
/// cargo names sits in are written nothing. Where cargo gives no root the
/// script prints one warning and writes nothing.
///
/// The crate's texts go straight to where they belong, standard output
/// or a file, through a sink the expansion defines, and the wrapper on
/// disk is held to its text by a sink that compares.
#[macro_export]
macro_rules! script {
    ($prototype:ident) => {{
        use ::core::fmt;
        use ::std::{env, fs, io, path::{Path, PathBuf}, process::Command}; // lint:allow(forbidden-imports) reason: the macro's expansion is the build script's code, compiled there and never in this crate; tracked: forbidden_imports_reads_the_body_of_an_exported_macro_as_the

        /// Hands the crate's text to a file or to standard output, and
        /// keeps the reason of a write that failed.
        struct Sink<W: io::Write> {
            to:     W,
            failed: io::Result<()>,
        }

        impl<W: io::Write> Sink<W> {
            fn over(to: W) -> Self {
                Sink {
                    to,
                    failed: Ok(()),
                }
            }

            fn finish(self, wrote: fmt::Result) -> io::Result<()> {
                self.failed?;
                wrote.map_err(|_| io::Error::other("the text was refused"))
            }
        }

        impl<W: io::Write> fmt::Write for Sink<W> {
            #[rustfmt::skip] // keeps the allow on the signature it governs
            fn write_str(&mut self, part: &str) -> fmt::Result { // lint:allow(no-bare-string) reason: the signature of `core::fmt::Write`; tracked: #72
                self.to.write_all(part.as_bytes()).map_err(|why| {
                    self.failed = Err(why);
                    fmt::Error
                })
            }
        }

        /// Compares what is written to it with what a file holds.
        struct Same<'a> {
            rest: &'a [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a file's bytes as the build script read them; tracked: #72
            same: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state in the build script; tracked: #72
        }

        impl fmt::Write for Same<'_> {
            #[rustfmt::skip] // keeps the allow on the signature it governs
            fn write_str(&mut self, part: &str) -> fmt::Result { // lint:allow(no-bare-string) reason: the signature of `core::fmt::Write`; tracked: #72
                match self.rest.strip_prefix(part.as_bytes()) {
                    Some(rest) => self.rest = rest,
                    None => self.same = false,
                }
                Ok(())
            }
        }

        let mut stdout = Sink::over(io::stdout());
        let cfg = $crate::Prototype::$prototype
            .configure(|key| env::var(key).ok().into(), &mut stdout)
            .expect("standard output takes the directives");

        let root = env::var_os("CARGO").and_then(|cargo| {
            let found = Command::new(cargo)
                .args(["locate-project", "--workspace", "--message-format", "plain"])
                .current_dir(env::var_os("CARGO_MANIFEST_DIR")?)
                .output()
                .ok()?;
            if !found.status.success() {
                return None;
            }
            let manifest = str::from_utf8(&found.stdout).ok()?;
            Path::new(manifest.trim()).parent().map(Path::to_path_buf)
        });
        match root {
            None => {
                println!(
                    "cargo::warning=hilavitkutin-build got no workspace root from cargo, so it looked at no wiring \
                     and wrote none"
                );
            },
            Some(root) => {
                let dir = root.join(".cargo");
                let wrapper_path = dir.join($crate::WRAPPER_NAME);
                let config_path = dir.join("config.toml");

                let cargo_home = env::var_os("CARGO_HOME")
                    .map(PathBuf::from)
                    .or_else(|| env::home_dir().map(|home| home.join(".cargo")));
                let named = env::var("RUSTC_WORKSPACE_WRAPPER").ok();
                // The wrapper cargo names is ours by its name, and this
                // root's by its place.
                let elsewhere = named.as_deref().map(Path::new).is_some_and(|named| {
                    let placed = |path: &Path| fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
                    named.file_name().is_some_and(|name| name == $crate::WRAPPER_NAME)
                        && placed(named) != placed(&wrapper_path)
                });
                let tree = if cargo_home.is_some_and(|home| root.starts_with(home))
                    || root.join("Cargo.toml.orig").exists()
                {
                    $crate::Tree::Fetched
                } else if elsewhere {
                    $crate::Tree::Foreign
                } else {
                    $crate::Tree::Own
                };

                let wrapper_file = match fs::read(&wrapper_path) {
                    Ok(have) => {
                        let mut held = Same {
                            rest: &have,
                            same: true,
                        };
                        let written = $crate::write_wrapper(&mut held).is_ok();
                        if written && held.same && held.rest.is_empty() {
                            $crate::WrapperFile::Current
                        } else {
                            $crate::WrapperFile::Stale
                        }
                    },
                    Err(_) if wrapper_path.exists() => $crate::WrapperFile::Stale,
                    Err(_) => $crate::WrapperFile::Missing,
                };
                let config_file = match fs::read_to_string(&config_path) {
                    Ok(have) => $crate::ConfigFile::read(&have),
                    Err(_) if config_path.exists() => $crate::ConfigFile::Unnamed,
                    Err(_) => $crate::ConfigFile::Missing,
                };

                let plan = $crate::plan(
                    named.as_deref().into(),
                    tree,
                    wrapper_file,
                    config_file,
                    $crate::flags::writes_flags(cfg.pragmas),
                );

                let wrote = (|| -> io::Result<()> {
                    if plan.wrapper || plan.config {
                        fs::create_dir_all(&dir)?;
                    }
                    if plan.wrapper {
                        let mut file = Sink::over(fs::File::create(&wrapper_path)?);
                        let wrote = $crate::write_wrapper(&mut file);
                        file.finish(wrote)?;
                        #[cfg(unix)]
                        {
                            use ::std::os::unix::fs::PermissionsExt as _; // lint:allow(forbidden-imports) reason: the macro's expansion is the build script's code, compiled there and never in this crate; tracked: forbidden_imports_reads_the_body_of_an_exported_macro_as_the
                            fs::set_permissions(&wrapper_path, fs::Permissions::from_mode(0o755))?;
                        }
                    }
                    if plan.config {
                        let mut file = Sink::over(fs::File::create_new(&config_path)?);
                        let wrote = $crate::write_cargo_config(&mut file);
                        file.finish(wrote)?;
                    }
                    Ok(())
                })();
                match wrote {
                    Ok(()) => {
                        $crate::write_notice(plan.notice, dir.display(), &mut stdout)
                            .expect("standard output takes the notice");
                    },
                    Err(why) => {
                        println!(
                            "cargo::warning=hilavitkutin-build could not write the wiring under {}: {why}",
                            dir.display()
                        );
                    },
                }
                if tree == $crate::Tree::Own {
                    $crate::write_watch(dir.display(), &mut stdout)
                        .expect("standard output takes the watch lines");
                }
            },
        }
    }};
}
