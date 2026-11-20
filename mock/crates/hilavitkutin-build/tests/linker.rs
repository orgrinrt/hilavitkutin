//! The linker of a set, what each linker leaves in what it links, and the
//! toolchain's own linker as rustc's description of a target gives it.

mod common;

use common::pragmas;
use hilavitkutin_build::{
    Bfd,
    ChecksumFreshness,
    ClangDriver,
    Group,
    Linker,
    LinkerGroup,
    Mold,
    NoIncremental,
    ToolchainLinker,
    Wild,
    linker_named,
    linker_of,
    rust_lld_mark,
};
use notko::Maybe;

const MOLD: Linker = Linker::Named {
    name: "Mold",
    mark: Maybe::Is("mold "),
};
const WILD: Linker = Linker::Named {
    name: "Wild",
    mark: Maybe::Is("Linker: Wild"),
};
const BFD: Linker = Linker::Named {
    name: "Bfd",
    mark: Maybe::Isnt,
};

#[test]
fn a_set_is_linked_by_its_linker_pragma_and_by_the_toolchain_where_it_holds_none() {
    assert_eq!(linker_of::<()>(), Linker::Toolchain);
    assert_eq!(
        linker_of::<(ChecksumFreshness, NoIncremental, ClangDriver)>(),
        Linker::Toolchain,
        "a link driver is not a linker"
    );
    assert_eq!(linker_of::<(Mold,)>(), MOLD);
    assert_eq!(linker_of::<(NoIncremental, Wild, ClangDriver)>(), WILD);
    assert_eq!(linker_of::<(Bfd, ChecksumFreshness)>(), BFD);
}

/// Every pragma of the linker's group is found by its name, so a pragma
/// added to the group and not to the list the walk reads fails here.
#[test]
fn every_linker_is_found_again_by_the_name_it_gives() {
    assert_eq!(linker_named("toolchain"), Maybe::Is(Linker::Toolchain));
    assert_eq!(Linker::Toolchain.name(), "toolchain");
    let mut linkers = 0;
    for row in pragmas() {
        let found = linker_named(row.name);
        if row.group == LinkerGroup::BIT {
            linkers += 1;
            let Maybe::Is(found) = found else {
                panic!("{} is a linker and no name finds it", row.name);
            };
            assert_eq!(found.name(), row.name);
        } else {
            assert_eq!(found, Maybe::Isnt, "{} is no linker", row.name);
        }
    }
    assert_eq!(linkers, 3);
    assert_eq!(linker_named("Mold"), Maybe::Is(MOLD));
    assert_eq!(linker_named("Wild"), Maybe::Is(WILD));
    assert_eq!(linker_named("Bfd"), Maybe::Is(BFD));
    for unknown in ["", "mold", "Gold", "default", "Toolchain"] {
        assert_eq!(linker_named(unknown), Maybe::Isnt, "{unknown:?}");
    }
}

const OWN: &str = "Linker: LLD 22.1.6 (/checkout/src/llvm-project/llvm 08c84e69)";

#[test]
fn a_named_linker_leaves_a_text_that_starts_with_its_mark() {
    let own = Maybe::Is(OWN);
    assert!(MOLD.leaves(Maybe::Is("mold 2.30.0 (compatible with GNU ld)"), own));
    assert!(WILD.leaves(Maybe::Is("Linker: Wild version 0.10.0"), own));
    // Another linker's mark, the toolchain's own among them, and none.
    assert!(!MOLD.leaves(Maybe::Is(OWN), own));
    assert!(!MOLD.leaves(Maybe::Is("Linker: Wild version 0.10.0"), own));
    assert!(!MOLD.leaves(Maybe::Is("moldy 1"), own));
    assert!(!MOLD.leaves(Maybe::Isnt, own));
    assert!(!WILD.leaves(Maybe::Is(OWN), own));
    assert!(!WILD.leaves(Maybe::Isnt, Maybe::Isnt));
}

#[test]
fn a_linker_with_no_mark_leaves_nothing() {
    assert!(BFD.leaves(Maybe::Isnt, Maybe::Is(OWN)));
    assert!(BFD.leaves(Maybe::Isnt, Maybe::Isnt));
    assert!(!BFD.leaves(Maybe::Is(OWN), Maybe::Is(OWN)));
    assert!(!BFD.leaves(Maybe::Is("mold 2.30.0"), Maybe::Isnt));
}

#[test]
fn the_toolchains_own_leaves_exactly_the_mark_the_caller_worked_out() {
    let own = Maybe::Is(OWN);
    assert!(Linker::Toolchain.leaves(Maybe::Is(OWN), own));
    let longer = format!("{OWN}0");
    assert!(!Linker::Toolchain.leaves(Maybe::Is(&longer), own));
    let joined = format!("{OWN} | mold 2.30.0 (compatible with GNU ld)");
    assert!(!Linker::Toolchain.leaves(Maybe::Is(&joined), own));
    assert!(!Linker::Toolchain.leaves(Maybe::Is("Linker: LLD 22.1.6"), own));
    assert!(!Linker::Toolchain.leaves(Maybe::Is("mold 2.30.0"), own));
    assert!(!Linker::Toolchain.leaves(Maybe::Isnt, own));
    // A toolchain linker the caller has no mark for is held by nothing.
    assert!(!Linker::Toolchain.leaves(Maybe::Is(OWN), Maybe::Isnt));
    assert!(!Linker::Toolchain.leaves(Maybe::Isnt, Maybe::Isnt));
}

#[test]
fn the_toolchain_links_with_its_own_lld_only_where_both_fields_say_so() {
    let own = r#"{ "link-self-contained": { "components": [ "linker" ] }, "linker-flavor": "gnu-lld-cc" }"#;
    assert_eq!(ToolchainLinker::of(own), ToolchainLinker::RustLld);
    let cc = r#"{ "linker-flavor": "gnu-cc" }"#;
    assert_eq!(ToolchainLinker::of(cc), ToolchainLinker::Other("gnu-cc"));
    // An lld the toolchain does not carry is the machine's.
    let system_lld = r#"{ "linker-flavor": "gnu-lld-cc" }"#;
    assert_eq!(
        ToolchainLinker::of(system_lld),
        ToolchainLinker::Other("gnu-lld-cc")
    );
    // The toolchain's own copy of a linker that is not lld.
    let own_other =
        r#"{ "link-self-contained": { "components": [ "linker" ] }, "linker-flavor": "gnu-cc" }"#;
    assert_eq!(
        ToolchainLinker::of(own_other),
        ToolchainLinker::Other("gnu-cc")
    );
    // Self-contained parts that are not the linker.
    let crt = r#"{ "link-self-contained": { "components": [ "crto", "libc" ] }, "linker-flavor": "gnu-lld-cc" }"#;
    assert_eq!(
        ToolchainLinker::of(crt),
        ToolchainLinker::Other("gnu-lld-cc")
    );
    assert_eq!(ToolchainLinker::of("{}"), ToolchainLinker::Other(""));
    assert_eq!(ToolchainLinker::of(""), ToolchainLinker::Other(""));
}

/// The fields are found as keys: a value that spells a key, and a key that
/// only ends as one does, name nothing.
#[test]
fn a_field_is_read_where_its_name_is_a_key() {
    let spelled_in_a_value = r#"{ "note": "linker-flavor", "x-linker-flavor": "gnu-lld-cc", "linker-flavor": "gnu-cc" }"#;
    assert_eq!(
        ToolchainLinker::of(spelled_in_a_value),
        ToolchainLinker::Other("gnu-cc")
    );
    // The description as rustc prints it: one key to a line.
    let printed = "{\n  \"arch\": \"x86_64\",\n  \"link-self-contained\": {\n    \"components\": [\n      \"linker\"\n    ]\n  },\n  \"linker-flavor\": \"gnu-lld-cc\",\n  \"os\": \"linux\"\n}\n";
    assert_eq!(ToolchainLinker::of(printed), ToolchainLinker::RustLld);
}

#[test]
fn the_mark_of_rust_lld_is_its_version_line_without_the_gnu_words() {
    let said =
        "LLD 22.1.6 (/checkout/src/llvm-project/llvm 08c84e69) (compatible with GNU linkers)";
    let Maybe::Is(mark) = rust_lld_mark(said) else {
        panic!("a version line of rust-lld has a mark");
    };
    assert_eq!(mark.to_string(), OWN);
    assert_eq!(
        rust_lld_mark("LLD 22.1.6"),
        Maybe::Isnt,
        "a line with none of the GNU flavor's words"
    );
    assert_eq!(
        rust_lld_mark("mold 2.30.0 (compatible with GNU linkers)"),
        Maybe::Isnt,
        "a rust-lld that is not lld"
    );
    assert_eq!(rust_lld_mark(""), Maybe::Isnt);
}

// The group's own bit is what the walk compares, so it is not zero.
const _: () = assert!(<LinkerGroup as Group>::BIT != 0);
