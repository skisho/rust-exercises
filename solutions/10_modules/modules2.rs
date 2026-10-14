// modules2.rs - solution
//
// `use` brings a path into scope, either an item itself (`APPLE`) or a
// module (`fruits`). The `veggies` module had to be made `pub` first.

mod delicious_snacks {
    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
    }
}

fn main() {
    use delicious_snacks::fruits::{self, APPLE};
    use delicious_snacks::veggies::CUCUMBER;

    println!("favorite snacks: {} and {}", APPLE, CUCUMBER);
    println!("also nice: {}", fruits::PEAR);
}
