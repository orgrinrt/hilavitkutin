//! `script!` as a consumer meets it: one line in a build script kept at a
//! path of the consumer's own, in a workspace that has never been wired.
//! Each test writes a scratch workspace and builds it with the cargo that
//! runs the suite, offline, with no workspace wrapper named and with a
//! rustc that writes down the arguments it was run with.

mod harness;

use std::fs;

use harness::{
    A_FLAG_OF_OURS,
    WRITTEN_CONFIG,
    build,
    build_with,
    scratch,
    workspace,
    wrapper_text,
    write_crate,
};

#[test]
fn the_first_build_wires_the_workspace_and_the_second_takes_the_flags() {
    let root = workspace("unwired");
    let wrapper = root.join(".cargo/hilavitkutin-rustc-wrapper");
    let config = root.join(".cargo/config.toml");
    assert!(!root.join(".cargo").exists());

    let first = build(&root);
    assert_eq!(
        fs::read_to_string(&wrapper).expect("the wrapper was written"),
        wrapper_text()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = fs::metadata(&wrapper)
            .expect("the wrapper is there")
            .permissions()
            .mode();
        assert_eq!(mode & 0o111, 0o111, "mode {mode:o}");
    }
    assert_eq!(
        fs::read_to_string(&config).expect("the config file was written"),
        WRITTEN_CONFIG
    );
    assert_eq!(
        first.said.matches("in place under").count(),
        1,
        "{}",
        first.said
    );
    let compiles = first.compiles_of("scratch_unwired");
    assert_eq!(compiles.len(), 1, "{}", first.rustc);
    assert!(!compiles[0].contains(A_FLAG_OF_OURS), "{}", compiles[0]);

    // Cargo now names the wrapper, and the crate is built again through it.
    let second = build(&root);
    assert!(!second.said.contains("warning:"), "{}", second.said);
    let compiles = second.compiles_of("scratch_unwired");
    assert_eq!(compiles.len(), 1, "{}", second.rustc);
    assert!(
        compiles[0].ends_with("-Zthreads=0 -Zshare-generics=y"),
        "{}",
        compiles[0]
    );
}

/// A workspace of eight members built with eight jobs, so the scripts run
/// side by side: several find no wiring at the same moment, and the rest
/// find a config file written by another with no wrapper named.
#[test]
fn several_members_wire_the_workspace_together_and_all_take_the_flags() {
    const MEMBERS: usize = 8;
    let base = scratch("several");
    let root = base.join("ws");
    fs::create_dir_all(&root).expect("the workspace");
    let names: Vec<String> = (0 .. MEMBERS).map(|n| format!("m{n}")).collect();
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
            names
                .iter()
                .map(|name| format!("\"{name}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .expect("the workspace's manifest");
    for name in &names {
        write_crate(&root.join(name), &format!("scratch-several-{name}"), "");
    }
    let wrapper = root.join(".cargo/hilavitkutin-rustc-wrapper");
    let config = root.join(".cargo/config.toml");

    let first = build_with(&root, &["-j", "8"]);
    assert_eq!(
        fs::read_to_string(&wrapper).expect("the wrapper was written"),
        wrapper_text()
    );
    assert_eq!(
        fs::read_to_string(&config).expect("the config file was written"),
        WRITTEN_CONFIG
    );
    // Nothing of a script's own is left beside the two files.
    let left: Vec<_> = fs::read_dir(root.join(".cargo"))
        .expect("the config directory")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert_eq!(left.len(), 2, "{left:?}");
    assert!(!first.said.contains("could not write"), "{}", first.said);
    let wired = first.said.matches("in place under").count();
    let dropped = first
        .said
        .matches("so they are dropped in this build")
        .count();
    assert!(wired >= 1, "{}", first.said);
    assert_eq!(wired + dropped, MEMBERS, "{}", first.said);

    let second = build_with(&root, &["-j", "8"]);
    assert!(!second.said.contains("warning:"), "{}", second.said);
    for name in &names {
        let compiles = second.compiles_of(&format!("scratch_several_{name}"));
        assert_eq!(compiles.len(), 1, "{name}: {}", second.rustc);
        assert!(compiles[0].ends_with(A_FLAG_OF_OURS), "{}", compiles[0]);
    }
}

/// The wrapper is held to the whole text and to nothing more: one that
/// starts with the text and goes on is not current.
#[test]
fn a_wrapper_with_a_line_added_after_the_text_is_written_again() {
    let root = workspace("longer");
    let wrapper = root.join(".cargo/hilavitkutin-rustc-wrapper");
    build(&root);
    build(&root);

    let longer = wrapper_text() + "# a line somebody added\n";
    fs::write(&wrapper, &longer).expect("the longer wrapper");
    build(&root);
    assert_eq!(
        fs::read_to_string(&wrapper).expect("the wrapper is there"),
        wrapper_text()
    );
}

#[test]
fn a_wrapper_that_holds_another_text_is_written_again() {
    let root = workspace("stale");
    let wrapper = root.join(".cargo/hilavitkutin-rustc-wrapper");
    build(&root);
    build(&root);

    // An older version's wrapper: rustc unchanged, the flags dropped.
    fs::write(&wrapper, "#!/bin/sh\nexec \"$@\"\n").expect("the stale wrapper");
    let stale = build(&root);
    assert_eq!(
        fs::read_to_string(&wrapper).expect("the wrapper is there"),
        wrapper_text()
    );
    assert!(!stale.said.contains("warning:"), "{}", stale.said);

    // Cargo dates a script's run from its start, so the wrapper the script
    // wrote during that run reads as changed once more and the next build
    // runs the script again. It finds the text in place and leaves the
    // file as it is: the file is the same one, written at the same moment.
    let written = fs::metadata(&wrapper)
        .and_then(|file| file.modified())
        .expect("the wrapper has a date");
    build(&root);
    build(&root);
    assert_eq!(
        fs::metadata(&wrapper)
            .and_then(|file| file.modified())
            .expect("the wrapper has a date"),
        written
    );
    assert_eq!(
        fs::read_to_string(&wrapper).expect("the wrapper is there"),
        wrapper_text()
    );
}

#[test]
fn a_config_file_without_the_line_is_left_as_it_is() {
    let root = workspace("own-config");
    let config = root.join(".cargo/config.toml");
    let theirs = "# the consumer's own\n[build]\nincremental = true\n";
    fs::create_dir_all(root.join(".cargo")).expect("the config directory");
    fs::write(&config, theirs).expect("the consumer's config");

    let built = build(&root);
    assert_eq!(
        fs::read_to_string(&config).expect("the config file"),
        theirs
    );
    assert!(
        built.said.contains(
            "add `rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"` under `[build]`"
        ),
        "{}",
        built.said
    );
    assert!(!built.said.contains("in place under"), "{}", built.said);
    assert_eq!(
        fs::read_to_string(root.join(".cargo/hilavitkutin-rustc-wrapper"))
            .expect("the wrapper was written"),
        wrapper_text()
    );
}

#[test]
fn an_unpacked_crate_is_written_nothing() {
    let root = workspace("unpacked");
    fs::write(
        root.join("Cargo.toml.orig"),
        "# what cargo keeps of the manifest it rewrote\n",
    )
    .expect("the marker of an unpacked crate");

    let built = build(&root);
    assert!(!root.join(".cargo").exists());
    assert!(!built.said.contains("warning:"), "{}", built.said);
}

#[test]
fn a_wiring_that_cannot_be_written_is_said_and_the_build_passes() {
    let root = workspace("unwritable");
    // A file where the directory would go, so nothing can be made under it.
    fs::write(root.join(".cargo"), "").expect("the file in the way");

    let built = build(&root);
    assert!(
        built
            .said
            .contains("hilavitkutin-build could not write the wiring under"),
        "{}",
        built.said
    );
    assert!(!built.said.contains("in place under"), "{}", built.said);
    assert!(root.join(".cargo").is_file());
}

/// A workspace under a directory whose own config file names the wrapper
/// there: cargo already runs through a wrapper of ours, and a second copy
/// inside the workspace would serve nothing.
#[test]
fn a_workspace_wired_from_above_is_written_nothing_and_takes_the_flags() {
    let root = workspace("from-above");
    let above = root
        .parent()
        .expect("the workspace sits in a scratch directory");
    fs::create_dir_all(above.join(".cargo")).expect("the config directory above");
    let wrapper = above.join(".cargo/hilavitkutin-rustc-wrapper");
    fs::write(&wrapper, wrapper_text()).expect("the wrapper above");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755)).expect("the wrapper runs");
    }
    fs::write(
        above.join(".cargo/config.toml"),
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
    )
    .expect("the config file above");

    let built = build(&root);
    assert!(!root.join(".cargo").exists());
    assert!(!built.said.contains("warning:"), "{}", built.said);
    let compiles = built.compiles_of("scratch_from_above");
    assert_eq!(compiles.len(), 1, "{}", built.rustc);
    assert!(compiles[0].ends_with(A_FLAG_OF_OURS), "{}", compiles[0]);
}

/// A path dependency that is a member of a second workspace. Cargo names
/// no workspace wrapper to the script of a crate outside the workspace it
/// was run in, so the script sees what the second workspace's own first
/// build would show it and wires that workspace, in the outer workspace's
/// first build and in any later one. The dependency takes no flags from
/// either. The design states this as the case it cannot tell apart, and
/// this test holds it as it is.
#[test]
fn a_path_dependency_in_another_workspace_has_that_workspace_wired() {
    let base = scratch("foreign");
    let sibling = base.join("sibling");
    fs::create_dir_all(&sibling).expect("the second workspace");
    fs::write(
        sibling.join("Cargo.toml"),
        "[workspace]\nmembers = [\"dep\"]\nresolver = \"3\"\n",
    )
    .expect("the second workspace's manifest");
    write_crate(&sibling.join("dep"), "scratch-foreign-dep", "");
    let root = base.join("ws");
    write_crate(
        &root,
        "scratch-foreign",
        "[dependencies]\nscratch-foreign-dep = { path = \"../sibling/dep\" }\n\n[workspace]\n",
    );

    let first = build(&root);
    assert!(root.join(".cargo/hilavitkutin-rustc-wrapper").exists());
    assert!(sibling.join(".cargo/hilavitkutin-rustc-wrapper").exists());
    let outside = first.compiles_of("scratch_foreign_dep");
    assert_eq!(outside.len(), 1, "{}", first.rustc);
    assert!(!outside[0].contains(A_FLAG_OF_OURS), "{}", outside[0]);

    // The outer workspace is wired now, and the script still cannot know.
    fs::remove_dir_all(sibling.join(".cargo")).expect("the second workspace's wiring goes");
    let second = build(&root);
    assert_eq!(
        fs::read_to_string(sibling.join(".cargo/hilavitkutin-rustc-wrapper"))
            .expect("the second workspace was wired again"),
        wrapper_text()
    );
    // The member takes the flags, the dependency outside the workspace none.
    let member = second.compiles_of("scratch_foreign");
    assert_eq!(member.len(), 1, "{}", second.rustc);
    assert!(member[0].ends_with(A_FLAG_OF_OURS), "{}", member[0]);
    let outside = second.compiles_of("scratch_foreign_dep");
    assert_eq!(outside.len(), 1, "{}", second.rustc);
    assert!(!outside[0].contains(A_FLAG_OF_OURS), "{}", outside[0]);
}
