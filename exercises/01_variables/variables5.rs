// variables5.rs
//
// Make this compile. An array's length is part of its type, so it has to
// be known when the program is *compiled*, not just when it runs. A `let`
// binding isn't good enough for that.
//
// Keep `board_area` as a function, and keep using it to size the board.

// I AM NOT DONE

fn board_area(width: usize, height: usize) -> usize {
    width * height
}

let WIDTH = 8;
let HEIGHT = 8;

fn main() {
    let squares = board_area(WIDTH, HEIGHT);
    let board = [' '; squares];
    println!("A {WIDTH}x{HEIGHT} board has {} squares.", board.len());
}
