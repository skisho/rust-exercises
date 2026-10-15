// variables3.rs
//
// Make this compile. It walks through a list of prices, keeping a running
// total and remembering the most expensive item seen so far.
//
// Only add what's needed: the compiler warns about anything you make
// mutable that never actually changes.

// I AM NOT DONE

fn main() {
    let prices = [4, 12, 7, 30, 2];
    let total = 0;
    let most_expensive = 0;

    for price in prices {
        total += price;
        if price > most_expensive {
            most_expensive = price;
        }
    }

    println!("{} items, total {total}, priciest {most_expensive}", prices.len());
}
