// functions2.rs - solution
//
// Unlike `let`, function parameters must always declare their type.

fn main() {
    countdown(3);
}

fn countdown(from: u32) {
    for n in (1..=from).rev() {
        println!("{n}...");
    }
    println!("Liftoff!");
}
