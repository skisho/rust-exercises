// variables1.rs - solution
//
// Each `let advice` above lived only until its closing brace. Declaring
// `advice` once, outside the `if`, keeps it alive for the `println!`.
// Because `if` is an expression, the cleanest fix assigns its result
// directly. (Declaring `let advice;` first and assigning in each branch
// also works: Rust checks that every path assigns it exactly once.)

fn main() {
    let raining = true;

    let advice = if raining {
        "take an umbrella"
    } else {
        "wear sunglasses"
    };

    println!("Today's advice: {advice}");
}
