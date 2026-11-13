Source: kaski channel (coord/kaski of clause-dev), att/20261113T182647263621690-hilavitkutin-and-kaski-take-hilavitkutin-build-with-a-first-3309b6/report-hilavitkutin-and-kaski-take-hilavitkutin-build-with-a-first-hilavitkutin.md

# hilavitkutin part: every crate builds through hilavitkutin-build

Branch `work/hilavitkutin-and-kaski-take-hilavitkutin-build-with-a-first` of `orgrinrt/hilavitkutin`,
round 202611131717 closed (topic, doc and src changelists under `mock/design_rounds/202611131717/`).

- `hilavitkutin-build`: `configure` / `configure_with` read `PROFILE` and `CARGO_CFG_TARGET_FEATURE`
  through the build script's closure and write the directives, now with
  `cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=<flags>`; `write_wrapper` owns the committed wrapper's
  text; `write_rustflags` writes each pragma's flags (LoopOptimization's five built-in llvm-args,
  `-Zthreads`, `-Zshare-generics`). Unused `arvo` dependency removed. `PROFILE` mapping fixed
  (cargo passes `debug`/`release` only).
- Design changes: FastMath owns no LLVM flag (`-enable-unsafe-fp-math` is refused by the pinned rustc),
  Pgo's flag renamed to `-loop-vectorize-with-block-frequency`, `-C target-cpu=native` removed from the
  always-on set, the plugin/container sources marked as not in the tree.
- Every other crate takes it as a build-dependency with a `build.rs`; `mock/.cargo/` holds the wrapper
  and the config naming it; `[profile.release]` fat LTO, one codegen unit, opt-level 3.

Tests: `hilavitkutin-build` 45 passed. Per crate on the branch: api 62 + 1 red, api-macros 2, ctx 3,
extensions 20, extensions-macros 5 + 1 red, kit 1, linking 10, sym 47, str 27, engine 236 + 1 red;
persistence and providers fail to link. All reds red on `origin/dev` 81c5c58 the same way.
Lints: all passed.

Tickets filed (workspace agenda, repos hilavitkutin):
- `hilavitkutin_build_build_scripts_find_the_plugins_and_profil` (FIXME at the plugin flag)
- `hilavitkutin_dev_tests_linking_hilavitkutin_str_fail_on_unde` (dev red)
- `hilavitkutin_dev_trybuild_compile_fail_fixtures_compile_and` (dev red)
- `hilavitkutin_build_the_design_s_pragma_trait_disagrees_with` (design versus source)
- `mockspace_close_in_hilavitkutin_sweeps_unrelated_flat_topics` (close swept two topics, moved back)

Review: three local passes by `pr-reviewer-senior`, last verdict merge. Its findings were answered in rounds
202611131745 (FastMath reach stated, `PragmaSet::coherent`, wrapper warning, `set -f`, real rustc probe with a
control, wrapper behaviour test, layering reasons, README), 202611131800 (wrapper recognised by its name, fourteen dead
`lint:allow` markers removed) and 202611131815 (design sentence on the one allowed const). Single reviewer seat on all
three passes; no second independent read was run.

More tickets:
- `arvo_should_a_crate_choose_fast_float_math_per_crate` (question, holder op)
- `forbidden_imports_does_not_read_build_scripts`
- `no_bare_static_str_does_not_know_a_build_dependency_only_cra`
- `a_lint_allow_after_an_opening_brace_is_moved_off_its_line_by`
- `the_commit_message_hook_builds_the_session_s_working_clone_s`
- `ticket_add_for_kaski_failed_to_push_to_dev_once`
- bumped `mockspace_close_in_hilavitkutin_sweeps_unrelated_flat_topics` (swept again twice, moved back)
