// variables4.rs
//
// Make this compile *without* adding `mut` and without renaming the
// second `input` variable. The first `input` is text; the second should
// be a number parsed from that text.

// I AM NOT DONE

fn main() {
    let input = "42";
    println!("the text is {input}");

    input = input.parse::<i32>().unwrap();
    println!("the number doubled is {}", input * 2);
}
