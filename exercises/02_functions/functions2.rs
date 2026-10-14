// functions2.rs
//
// Make this compile. Function parameters work a bit differently from
// `let` bindings.

// I AM NOT DONE

fn main() {
    countdown(3);
}

fn countdown(from) {
    for n in (1..=from).rev() {
        println!("{n}...");
    }
    println!("Liftoff!");
}
