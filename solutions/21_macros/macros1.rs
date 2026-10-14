// macros1.rs - solution
//
// Macro calls need a `!`. Unlike functions, `macro_rules!` macros are
// only visible *after* their definition in the file.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
