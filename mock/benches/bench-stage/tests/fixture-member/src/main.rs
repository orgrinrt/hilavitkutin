//! Prints the stage it was started from and whether it verifies, so the test
//! reads what a staged driver sees of itself.

fn main() {
    match bench_stage::Stage::of_current_exe() {
        Ok(stage) => {
            match stage.verify() {
                Ok(()) => println!("staged {}", stage.dir().display()),
                Err(e) => {
                    println!("changed {e}");
                    std::process::exit(2);
                },
            }
        },
        Err(e) => {
            println!("unstaged {e}");
            std::process::exit(3);
        },
    }
}
