// variables2.rs - solution
//
// A declared-but-uninitialised variable can't be read. Give it a value.

fn main() {
    let x: i32 = 10;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
