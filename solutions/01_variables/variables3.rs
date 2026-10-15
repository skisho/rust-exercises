// variables3.rs - solution
//
// `total` and `most_expensive` change inside the loop, so they need `mut`.
// `prices` never changes, so it stays immutable. Making things mutable
// only where needed tells readers which values to keep an eye on.

fn main() {
    let prices = [4, 12, 7, 30, 2];
    let mut total = 0;
    let mut most_expensive = 0;

    for price in prices {
        total += price;
        if price > most_expensive {
            most_expensive = price;
        }
    }

    println!("{} items, total {total}, priciest {most_expensive}", prices.len());
}
