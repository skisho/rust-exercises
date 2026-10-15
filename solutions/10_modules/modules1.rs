// modules1.rs - solution
//
// `pub struct` makes the type visible, but each field is still private
// unless it is marked `pub` too. Because `pin` is private, code outside
// the module can't build a `Vault` with a struct literal, so the module
// offers a public constructor instead. This is how Rust enforces
// invariants: only the module's own code can touch the private fields.

mod vault {
    pub struct Vault {
        pub owner: String,
        pin: u32,
    }

    impl Vault {
        pub fn new(owner: &str, pin: u32) -> Self {
            Vault {
                owner: owner.to_string(),
                pin,
            }
        }

        pub fn unlock(&self, attempt: u32) -> bool {
            attempt == self.pin
        }
    }
}

fn main() {
    let v = vault::Vault::new("Ferris", 1234);

    println!("{}'s vault", v.owner);
    println!("1111 opens it? {}", v.unlock(1111));
    println!("1234 opens it? {}", v.unlock(1234));
}
