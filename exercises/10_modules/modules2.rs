// modules2.rs
//
// Make this compile by adding `use` declarations at the top of `main`
// (don't change the rest of `main`). You may also need to change
// visibility inside the modules.

// I AM NOT DONE

mod delicious_snacks {
    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
    }
}

fn main() {
    // TODO: add `use` declarations here

    println!("favorite snacks: {} and {}", APPLE, CUCUMBER);
    println!("also nice: {}", fruits::PEAR);
}
