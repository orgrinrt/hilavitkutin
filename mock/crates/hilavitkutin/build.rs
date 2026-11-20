//! Builds the crate through `hilavitkutin-build`, and names the pragmas
//! that reach the whole workspace: this is the one script that claims it.
fn main() {
    hilavitkutin_build::script!(Workspace, workspace);
}
