//! The part of a build script that finds and builds tools.
//!
//! [`script!`](crate::script) expands this beside its own body. What to do
//! about a tool is [`provision`](crate::provision)'s to answer; this is
//! the looking on the machine, the local build and the removing of older
//! versions, which only the script can do.

/// The functions `script!` finds and builds a tool with.
///
/// `provide` answers the path of a tool, or nothing where the build goes
/// on without one. Where the machine lacks a tool the script does not
/// build, or a local build fails, it prints a `cargo::error`, so the
/// build fails once the script has run to its end.
#[doc(hidden)]
#[macro_export]
macro_rules! __script_tools {
    () => {
        /// The first line a program prints when asked for its version.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn version_line(program: &Path) -> Option<impl AsRef<str> + use<>> { // lint:allow(no-bare-option) lint:allow(no-bare-string) reason: a version line as the build script read it, in std's own terms; tracked: #72
            let asked = Command::new(program).arg("--version").output().ok()?;
            if !asked.status.success() {
                return None;
            }
            str::from_utf8(&asked.stdout).ok()?.lines().next().map(str::to_owned)
        }

        /// Whether the program at `program` is `tool`, by its version line.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn is_tool(tool: &$crate::Tool, program: &Path) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a predicate of the build script; tracked: #72
            version_line(program).is_some_and(|line| tool.proof.holds(line.as_ref()))
        }

        /// Whether the toolchain's sysroot holds the codegen backend `name`.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn in_sysroot(name: &'static str) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a predicate of the build script; tracked: #72
            let found = (|| {
                let asked = Command::new(env::var_os("RUSTC")?).args(["--print", "sysroot"]).output().ok()?;
                let sysroot = PathBuf::from(str::from_utf8(&asked.stdout).ok()?.trim());
                let backends = sysroot.join("lib/rustlib").join(env::var_os("HOST")?).join("codegen-backends");
                let wanted = format!("rustc_codegen_{name}");
                let held = fs::read_dir(backends).ok()?.flatten();
                Some(held.into_iter().any(|entry| entry.file_name().to_string_lossy().contains(&wanted)))
            })();
            found.unwrap_or(false)
        }

        /// The directory of `tool` under the tools home, where there is a home.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn tool_dir(tool: &$crate::Tool) -> Option<PathBuf> { // lint:allow(no-bare-option) reason: the build script's own std code; tracked: #72
            let named = env::var("HILAVITKUTIN_BUILD_TOOLS_HOME").ok();
            let xdg = env::var("XDG_CACHE_HOME").ok();
            let user = env::home_dir();
            let user = user.as_deref().map(Path::to_string_lossy);
            let host = if cfg!(target_os = "macos") { $crate::Os::MacOs } else { $crate::Os::Other };
            let home = $crate::tools_home(named.as_deref().into(), xdg.as_deref().into(), user.as_deref().into(), host);
            let home = Option::<$crate::Home<'_>>::from(home)?; // lint:allow(no-bare-option) reason: the build script's own std code; tracked: #72
            let triple = env::var("HOST").ok()?;
            let mut dir = "".to_owned();
            $crate::write_tool_dir(home, &triple, tool, &mut dir).ok()?;
            Some(PathBuf::from(dir))
        }

        /// The tools this script has said the machine lacks.
        static REFUSED: ::std::sync::Mutex<[&'static str; 16]> = ::std::sync::Mutex::new([""; 16]); // lint:allow(no-bare-static-str) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the build script's own std code, a set has sixteen tools at most; tracked: #72

        /// Find `tool`, or build it where it is a local build and is on
        /// the machine nowhere.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn provide(tool: &'static $crate::Tool) -> Option<PathBuf> { // lint:allow(no-bare-option) reason: the build script's own std code; tracked: #72
            let on_path = match tool.proof.line {
                $crate::Line::Sysroot => in_sysroot(tool.name).then(|| "".to_owned()),
                _ => version_line(Path::new(tool.name)).map(|line| line.as_ref().to_owned()),
            };
            let dir = tool_dir(tool);
            let program = dir.as_ref().map(|dir| dir.join("bin").join(tool.name));
            let in_home = program.as_deref().and_then(version_line);
            let in_home = in_home.as_ref().map(AsRef::as_ref);
            match $crate::provision(tool, on_path.as_deref().into(), in_home.into()) {
                $crate::Provision::Awaited => None,
                $crate::Provision::Path => Some(PathBuf::from(tool.name)),
                $crate::Provision::Home => program,
                $crate::Provision::Refuse => {
                    // A tool is asked for by the profile's set and again
                    // by the workspace's check, and refused in one line.
                    let mut said = REFUSED.lock().unwrap_or_else(|held| held.into_inner());
                    if !said.contains(&tool.name) {
                        if let Some(free) = said.iter_mut().find(|name| name.is_empty()) {
                            *free = tool.name;
                        }
                        let mut stdout = Sink::over(io::stdout());
                        $crate::write_refusal(tool, &mut stdout).expect("standard output takes the refusal");
                    }
                    None
                },
                $crate::Provision::Build => {
                    let pin = Option::<&'static str>::from(tool.version); // lint:allow(no-bare-option) reason: the build script's own std code; tracked: #72
                    match (dir, program, tool.source, pin) {
                        (Some(dir), Some(program), $crate::Source::LocalBuild(krate), Some(pin)) => {
                            build(tool, krate, pin, &dir, &program).then_some(program)
                        },
                        _ => {
                            println!(
                                "cargo::error=hilavitkutin-build: {} is built on the machine and there is no tools \
                                 home to build it into: set HILAVITKUTIN_BUILD_TOOLS_HOME, or HOME",
                                tool.name
                            );
                            None
                        },
                    }
                },
            }
        }

        /// Say that the local build of `name` failed, and why.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn build_failed<D: fmt::Display>(name: &'static str, pin: &'static str, dir: &Path, why: D) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: whether the build script has the tool; tracked: #72
            println!(
                "cargo::error=hilavitkutin-build could not build {name} {pin} into {}: {why}",
                dir.display()
            );
            false
        }

        /// Build `tool` into `dir` with `cargo install`, and remove the
        /// older versions beside it. Scripts of one workspace run side by
        /// side, so the one that builds holds a lock and the others wait
        /// for it, and then find the tool built.
        #[rustfmt::skip] // keeps the allow on the signature it governs
        fn build(tool: &$crate::Tool, krate: &'static str, pin: &'static str, dir: &Path, program: &Path) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: whether the build script has the tool; tracked: #72
            let name = tool.name;
            let Some(beside) = dir.parent() else {
                return build_failed(name, pin, dir, "the tools home has no directory for it");
            };
            if let Err(why) = fs::create_dir_all(beside) {
                return build_failed(name, pin, dir, why);
            }
            // The lock is the operating system's, on a file that stays: it
            // goes with the process that holds it, so a build that was
            // interrupted holds nobody up.
            let lock = beside.join(format!("{name}-{pin}.lock"));
            let held = match fs::File::create(&lock).and_then(|file| file.lock().map(|()| file)) {
                Ok(held) => held,
                Err(why) => {
                    return build_failed(name, pin, dir, format_args!("the lock {} was not taken: {why}", lock.display()));
                },
            };
            // The lock is held from here. Another script may have built
            // the tool while this one waited for it.
            let built = is_tool(tool, program) || {
                println!(
                    "cargo::warning=hilavitkutin-build is building {name} {pin} into {} with cargo install, once \
                     for this machine, which takes minutes",
                    dir.display()
                );
                let scratch = dir.join("build");
                // The child is a build of its own: its own target directory,
                // and none of this build's wrapper or flags.
                let ran = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
                    .args(["install", "--locked", "--version", pin, "--root"])
                    .arg(dir)
                    .arg(krate)
                    .env("CARGO_TARGET_DIR", &scratch)
                    .env_remove("RUSTC_WORKSPACE_WRAPPER")
                    .env_remove("RUSTC_WRAPPER")
                    .env_remove("CARGO_ENCODED_RUSTFLAGS")
                    .env_remove("RUSTFLAGS")
                    .env_remove("CARGO_BUILD_TARGET")
                    .current_dir(beside)
                    .output();
                let _ = fs::remove_dir_all(&scratch);
                let installed = ran.is_ok_and(|ran| ran.status.success()) && is_tool(tool, program);
                if installed {
                    // A pin that moved up leaves no older version behind. A
                    // newer one is another clone's pin and stays.
                    let prefix = format!("{name}-");
                    for other in fs::read_dir(beside).into_iter().flatten().flatten() {
                        let other = other.path();
                        let below = other
                            .file_name()
                            .and_then(|file| file.to_str())
                            .and_then(|file| file.strip_prefix(&prefix))
                            .is_some_and(|version| $crate::older(version, pin));
                        if below {
                            let _ = fs::remove_dir_all(&other);
                        }
                    }
                }
                installed
            };
            drop(held);
            built
                || build_failed(name, pin, dir, format_args!(
                    "run `cargo install --locked --version {pin} --root {} {krate}` to see why",
                    dir.display()
                ))
        }
    };
}
