// intro2.rs - solution
//
// `{}` uses the `Display` trait, which is for user-facing output, and
// tuples and arrays don't implement it. `{:?}` uses `Debug` instead, which
// almost every standard type implements. (`{:#?}` pretty-prints it.)

fn main() {
    let position = (3, -7);
    let path = ["north", "north", "east"];

    println!("The robot is at {:?}", position);
    println!("It walked {:?}", path);
}
