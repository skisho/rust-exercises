// variables1.rs
//
// Make this compile. A variable only exists inside the block (the pair of
// curly braces) where it was declared. Don't change the `println!` lines.

// I AM NOT DONE

fn main() {
    let raining = true;

    if raining {
        let advice = "take an umbrella";
    } else {
        let advice = "wear sunglasses";
    }

    println!("Today's advice: {advice}");
}
