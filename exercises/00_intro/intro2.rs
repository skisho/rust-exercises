// intro2.rs
//
// Make this program compile. Read the compiler's error message carefully:
// Rust's errors usually tell you exactly what's wrong, and often suggest
// the fix.
//
// Numbers and strings know how to print themselves for users, but a tuple
// doesn't. It can still be printed in a programmer-facing "debug" form.

// I AM NOT DONE

fn main() {
    let position = (3, -7);
    let path = ["north", "north", "east"];

    println!("The robot is at {}", position);
    println!("It walked {}", path);
}
