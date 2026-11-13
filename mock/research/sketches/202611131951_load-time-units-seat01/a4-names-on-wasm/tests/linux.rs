//! Control: the same code, native on linux, resolves its own const name.

#[test]
fn control_the_same_image_on_linux_resolves_its_own_const_name() {
    assert!(a4_names_on_wasm::section_entries() >= 1);
    assert_eq!(a4_names_on_wasm::own_name_resolves(), 1);
}
