// macros1.rs
//
// Make this compile. Macros are invoked a little differently from
// functions, and must be defined before they're used.

// I AM NOT DONE

fn main() {
    my_macro();
}

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
