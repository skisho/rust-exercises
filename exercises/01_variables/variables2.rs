// variables2.rs
//
// Make this compile. When you don't write a type, Rust picks one for you:
// whole numbers default to `i32`. That's not always big enough.
//
// Fix it by adding type annotations, not by changing any numbers.

// I AM NOT DONE

fn main() {
    let world_population = 8_100_000_000;
    let bytes_in_a_terabyte = 1_000_000_000_000;
    let temperature = -40;

    println!("people:      {world_population}");
    println!("bytes in TB: {bytes_in_a_terabyte}");
    println!("cold:        {temperature}");
}
