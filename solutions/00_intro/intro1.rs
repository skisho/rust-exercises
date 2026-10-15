// intro1.rs - solution
//
// Nothing to fix: deleting the marker is the whole exercise. `1..=crabs`
// is an inclusive range, and `{i}` in a format string prints the variable
// `i` directly.

fn main() {
    let crabs = 3;
    for i in 1..=crabs {
        println!("crab #{i} reporting for duty 🦀");
    }
    println!("Delete the marker comment in intro1.rs to move on.");
}
