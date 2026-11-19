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
/// A bare name is one of the crate's own prototypes, and a path is a
/// consumer's: `script!(my_build::Sim)`. The one script of a workspace
/// that names the pragmas reaching all of it is written with
/// `, workspace` after the prototype.
///
/// The expansion holds `const` items that assert over the prototype's
/// sets, so a set that breaks a requirement, or names a pragma of
/// workspace reach in a script that does not claim the workspace, is
/// refused by a check at the script's line.
///
/// When it runs, the script resolves the build, finds the tools the
/// profile's set waits on (building one that is a local build and is on
/// the machine nowhere), and prints the set's directives. It asks cargo
/// for the workspace root (`cargo locate-project --workspace`, from
/// `CARGO_MANIFEST_DIR`), reads the wrapper and the `config.toml` under
/// its `.cargo/`, and does what [`plan`](crate::plan) answers: it writes
/// the wrapper where the file is missing or holds another text, writes
/// the config file where there is none, prints the notice, and asks to be
/// run again when either file changes. A root under `CARGO_HOME`, one
/// that holds `Cargo.toml.orig`, and one other than the root the wrapper
/// cargo names sits in are written nothing. Where cargo gives no root the
/// script prints one warning and writes nothing. The script that claims
/// the workspace then keeps the generated cargo file to the text of
/// [`write_generated`](crate::write_generated).
///
/// The crate's texts go straight to where they belong, standard output
/// or a file, through a sink the expansion defines, and a file on disk is
/// held to its text by a sink that compares.
#[macro_export]
macro_rules! script {
    ($prototype:ident) => {
        $crate::__script!(crate, $crate::prototype::$prototype)
    };
    ($prototype:ident,workspace) => {
        $crate::__script!(workspace, $crate::prototype::$prototype)
    };
    ($prototype:ty) => {
        $crate::__script!(crate, $prototype)
    };
    ($prototype:ty,workspace) => {
        $crate::__script!(workspace, $prototype)
    };
}

/// The body of [`script!`], and its parts that differ by whether the
/// script claims the workspace.
#[doc(hidden)]
#[macro_export]
macro_rules! __script {
    (@checks crate, $p:ty) => {
        const _: () = {
            $crate::checked::<<$p as $crate::Prototype>::Dev>();
            $crate::crate_reach::<<$p as $crate::Prototype>::Dev>();
        };
        const _: () = {
            $crate::checked::<<$p as $crate::Prototype>::DevOpt>();
            $crate::crate_reach::<<$p as $crate::Prototype>::DevOpt>();
        };
        const _: () = {
            $crate::checked::<<$p as $crate::Prototype>::Release>();
            $crate::crate_reach::<<$p as $crate::Prototype>::Release>();
        };
        const _: () = {
            $crate::checked::<<$p as $crate::Prototype>::Profiling>();
            $crate::crate_reach::<<$p as $crate::Prototype>::Profiling>();
        };
        const _: () = {
            $crate::checked::<<$p as $crate::Prototype>::Ci>();
            $crate::crate_reach::<<$p as $crate::Prototype>::Ci>();
        };
    };
    (@checks workspace, $p:ty) => {
        const _: () = $crate::claimed::<$p>();
    };
    (@generated crate, $p:ty, $dir:ident, $tree:ident, $config:ident, $plan:ident, $stdout:ident) => {};
    (@generated workspace, $p:ty, $dir:ident, $tree:ident, $config:ident, $plan:ident, $stdout:ident) => {
        if $tree == $crate::Tree::Own {
            let path = $dir.join($crate::GENERATED_NAME);
            let krate = env::var("CARGO_PKG_NAME").unwrap_or_default();
            let triple = env::var("TARGET").unwrap_or_default();
            let have = fs::read(&path).ok();
            let text = have.as_deref().map(|bytes| str::from_utf8(bytes).unwrap_or_default());
            match $crate::claim(text.into(), &krate) {
                $crate::Claim::Theirs(other) => {
                    let other = if other.is_empty() { "somebody else" } else { other };
                    println!(
                        "cargo::error=hilavitkutin-build: {krate} claims the workspace with `script!(.., workspace)` \
                         and {} was written for {other}: one crate of a workspace claims it, so take \
                         `, workspace` out of one of the two scripts, or remove the file where nobody claims it",
                        path.display()
                    );
                },
                claim => {
                    let current = claim == $crate::Claim::Mine && {
                        let mut held = Same {
                            rest: have.as_deref().unwrap_or_default(),
                            same: true,
                        };
                        let written = $crate::write_generated::<$p, _>(&krate, &triple, &mut held).is_ok();
                        written && held.same && held.rest.is_empty()
                    };
                    let wrote = (|| -> io::Result<()> {
                        if current {
                            return Ok(());
                        }
                        fs::create_dir_all(&$dir)?;
                        let whole = $dir.join(format!(".{}.{}", $crate::GENERATED_NAME, process::id()));
                        let mut file = Sink::over(fs::File::create(&whole)?);
                        let wrote = $crate::write_generated::<$p, _>(&krate, &triple, &mut file);
                        file.finish(wrote)?;
                        fs::rename(&whole, &path)
                    })();
                    match wrote {
                        Ok(()) if current => {},
                        Ok(()) => {
                            $crate::write_notice($crate::Notice::Generated, $dir.display(), &mut $stdout)
                                .expect("standard output takes the notice");
                        },
                        Err(why) => {
                            println!(
                                "cargo::warning=hilavitkutin-build could not write {}: {why}",
                                path.display()
                            );
                        },
                    }
                    // A config file this script just wrote has the line.
                    let included = match &$config {
                        Some(text) => $crate::includes_generated(text),
                        None => $plan.config,
                    };
                    if !included {
                        $crate::write_notice($crate::Notice::AddInclude, $dir.display(), &mut $stdout)
                            .expect("standard output takes the notice");
                    }
                },
            }
            println!("cargo::rerun-if-changed={}", path.display());
        }
    };
    ($claim:ident, $p:ty) => {{
        use ::core::fmt;
        use ::std::{
            env,
            fs,
            io,
            path::{Path, PathBuf},
            process::{self, Command},
            thread,
            time::{Duration, Instant},
        };

        $crate::__script!(@checks $claim, $p);

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

        $crate::__script_tools!();

        /// Print the directives of the profile's set, with the tools it
        /// waits on found first, and say whether it has flags.
        fn directives<S: $crate::Set>(cfg: &$crate::BuildConfig, stdout: &mut Sink<io::Stdout>) -> $crate::Flags {
            // A set holds sixteen pragmas at most, so sixteen tools.
            let mut found: [_; 16] = ::core::array::from_fn(|_| None);
            let mut count = 0;
            $crate::each_tool::<S>(|tool| {
                let seen = found[..count].iter().flatten().any(|(have, _): &(&$crate::Tool, PathBuf)| have.name == tool.name);
                if !seen && let Some(path) = provide(tool) {
                    found[count] = Some((tool, path));
                    count += 1;
                }
            });
            let texts: [_; 16] = ::core::array::from_fn(|at| found[at].as_ref().map(|(_, path)| path.to_string_lossy()));
            let mut paths = [$crate::ToolPath::new(&$crate::tools::MOLD, ""); 16];
            for at in 0..count {
                if let (Some((tool, _)), Some(text)) = (&found[at], &texts[at]) {
                    paths[at] = $crate::ToolPath::new(tool, text);
                }
            }
            let triple = env::var("TARGET").unwrap_or_default();
            let site = $crate::Site::new(cfg.profile, &triple, &paths[..count]);
            $crate::write_directives::<S, _>(cfg, &site, stdout).expect("standard output takes the directives");
            $crate::writes_flags::<S>(&site)
        }

        let mut stdout = Sink::over(io::stdout());
        let cfg = $crate::BuildConfig::from_env::<$p, _>(|key| env::var(key).ok().into());
        let flags = match cfg.profile {
            $crate::Profile::Dev => directives::<<$p as $crate::Prototype>::Dev>(&cfg, &mut stdout),
            $crate::Profile::DevOpt => directives::<<$p as $crate::Prototype>::DevOpt>(&cfg, &mut stdout),
            $crate::Profile::Release => directives::<<$p as $crate::Prototype>::Release>(&cfg, &mut stdout),
            $crate::Profile::Profiling => directives::<<$p as $crate::Prototype>::Profiling>(&cfg, &mut stdout),
            $crate::Profile::Ci => directives::<<$p as $crate::Prototype>::Ci>(&cfg, &mut stdout),
        };

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
                let cargo_home = cargo_home.as_deref().map(Path::to_string_lossy);
                let tree = $crate::Tree::of(
                    &root.to_string_lossy(),
                    cargo_home.as_deref().into(),
                    if root.join("Cargo.toml.orig").exists() { $crate::Unpacked::Is } else { $crate::Unpacked::Isnt },
                    if elsewhere { $crate::Placed::Elsewhere } else { $crate::Placed::Here },
                    if cfg!(unix) { $crate::Host::Unix } else { $crate::Host::Other },
                );

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
                let config_text = fs::read_to_string(&config_path).ok();
                let config_file = match &config_text {
                    Some(have) => $crate::ConfigFile::read(have),
                    None if config_path.exists() => $crate::ConfigFile::Unnamed,
                    None => $crate::ConfigFile::Missing,
                };

                let plan = $crate::plan(named.as_deref().into(), tree, wrapper_file, config_file, flags);

                let wrote = (|| -> io::Result<()> {
                    if plan.wrapper || plan.config {
                        fs::create_dir_all(&dir)?;
                    }
                    // Scripts of one workspace run side by side. Each writes
                    // under a name of its own and moves the whole file into
                    // place, so none reads half of what another wrote.
                    let mine = |name: &'static str| dir.join(format!(".{name}.{}", process::id()));
                    if plan.wrapper {
                        let whole = mine($crate::WRAPPER_NAME);
                        let mut file = Sink::over(fs::File::create(&whole)?);
                        let wrote = $crate::write_wrapper(&mut file);
                        file.finish(wrote)?;
                        #[cfg(unix)]
                        {
                            use ::std::os::unix::fs::PermissionsExt as _;
                            fs::set_permissions(&whole, fs::Permissions::from_mode(0o755))?;
                        }
                        fs::rename(&whole, &wrapper_path)?;
                    }
                    if plan.config {
                        let whole = mine("config.toml");
                        let mut file = Sink::over(fs::File::create(&whole)?);
                        let wrote = $crate::write_cargo_config(&mut file);
                        file.finish(wrote)?;
                        // A link refuses where a file is already there, so a
                        // config file is never one this script wrote over.
                        let placed = fs::hard_link(&whole, &config_path);
                        fs::remove_file(&whole)?;
                        match placed {
                            Err(why) if why.kind() != io::ErrorKind::AlreadyExists => return Err(why),
                            _ => {},
                        }
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
                $crate::__script!(@generated $claim, $p, dir, tree, config_text, plan, stdout);
            },
        }
    }};
}
