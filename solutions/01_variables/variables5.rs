// variables5.rs - solution
//
// `let` isn't allowed outside a function at all; global values must be
// `const` (or `static`), and constants always need a type annotation.
//
// An array length must be a constant expression. A plain `fn` call isn't
// one, but marking the function `const fn` lets the compiler evaluate it
// at compile time, so its result can be stored in a `const` and used as a
// length.

const fn board_area(width: usize, height: usize) -> usize {
    width * height
}

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

fn main() {
    const SQUARES: usize = board_area(WIDTH, HEIGHT);
    let board = [' '; SQUARES];
    println!("A {WIDTH}x{HEIGHT} board has {} squares.", board.len());
}
