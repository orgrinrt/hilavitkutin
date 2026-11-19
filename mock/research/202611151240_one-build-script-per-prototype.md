# One build script per prototype, and a pack that writes the wiring

15 November 2026. Holds for: Linux and macOS hosts, cargo on nightly-2026-05-28, a consumer workspace that runs mockspace. Windows is not covered (`BACKLOG.md.tmpl`, "Windows wrapper").

Op asked why a workspace taking `hilavitkutin-build` writes its wiring by hand, and why every crate repeats the same build script. Counted on 15 November: hilavitkutin holds fourteen byte-identical `build.rs` files, and kaski twenty-five copies of one deterministic script, seven of one shader script and one that is the deterministic script with a commit stamp. Beside them each consumer commits the wrapper, its `.cargo/config.toml` and a test holding the wrapper to `write_wrapper`.

## What cargo allows

The sketch `sketches/shared-build-script/` settles the mechanism. A member may name a script outside itself with `package.build`, and it builds and reruns only when its directives say so; `cargo package` refuses it, so a published crate cannot. A `build.rs` that is a symbolic link to the shared file builds the same and packages with the file's content. A composite script reached through a link cannot use `#[path]`, which resolves from the link's own directory, so a project prototype is one self-contained file.

## The vehicle chosen

A crate names its prototype in `[package.metadata.hilavitkutin-build]`, and its `build.rs` is a link into `<mock>/build/`. `Prototype` in `hilavitkutin-build` carries the crate's own prototypes and writes their text, so the shared files are generated rather than kept. A lint pack beside the crate, `hilavitkutin-build-pack`, has a maker tool that writes everything the choices imply (prototype files, links, build-dependencies, the ssh dependency, the wrapper with its executable bit and the cargo config) and a lint that computes the same plan and refuses a tree that differs from it.

Considered and not taken: `package.build` alone, refused by packaging for every publishable crate; a `build.rs` generated into each crate, which keeps a copy per crate and only moves the drift check; a cargo subcommand or a `build.rs` of the workspace, which cargo does not have; a lint without a maker, which refuses drift and leaves the writing by hand, the thing op asked to end.

## Status

The round `202611151157` carrying it is at its doc changelist. On 15 November its commits were refused by the bench lints mockspace landed that day, ticket `hilavitkutin_kirjo_and_arvo_port_their_benches_to_the_one_mo`. On 19 November, with `dev` merged in at `4acfbc7a`, the lint pipeline is green on the branch and the changelist names where the pack lives and the branch the round supersedes.
