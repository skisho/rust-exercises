// variables3.rs - solution
//
// Variables are immutable by default. `let mut` opts in to mutation.

fn main() {
    let mut count = 0;
    println!("count starts at {count}");

    count = count + 1;
    println!("count is now {count}");
}
