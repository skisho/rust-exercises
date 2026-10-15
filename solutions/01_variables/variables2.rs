// variables2.rs - solution
//
// An `i32` holds values up to about 2.1 billion, so both big literals
// overflow it, and Rust refuses to compile that. A 64-bit type fits them.
// `u64` is a good choice for counts that can't be negative. The
// temperature is fine as an `i32` (and must be signed!). You could also
// write the type on the literal itself: `8_100_000_000u64`.

fn main() {
    let world_population: u64 = 8_100_000_000;
    let bytes_in_a_terabyte: u64 = 1_000_000_000_000;
    let temperature = -40;

    println!("people:      {world_population}");
    println!("bytes in TB: {bytes_in_a_terabyte}");
    println!("cold:        {temperature}");
}
