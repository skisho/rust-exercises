// intro1.rs
//
// Welcome! Each exercise is a small Rust program that either doesn't
// compile, or compiles but fails its tests. Your job is to fix it.
//
// While you work, keep `cargo run -- watch` running in a terminal. It
// re-checks your progress every time you save a file.
//
// This first exercise already works. Try `cargo run -- run intro1` to see
// its output, and change the number of crabs if you like.
//
// When an exercise passes, the runner still waits for you to delete the
// marker comment below. That gives you time to experiment. Delete it to
// continue.
//
// Stuck? Run `cargo run -- hint intro1`.

// I AM NOT DONE

fn main() {
    let crabs = 3;
    for i in 1..=crabs {
        println!("crab #{i} reporting for duty 🦀");
    }
    println!("Delete the marker comment in intro1.rs to move on.");
}
