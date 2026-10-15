// modules1.rs
//
// Make this compile. Everything inside a module is private unless it's
// marked `pub`, and that includes the *fields* of a struct.
//
// The vault should keep its PIN secret: code outside the `vault` module
// must not be able to read or change `pin` directly. So don't make `pin`
// public. Make only what `main` needs public, and give the module a way
// to create a `Vault`.

// I AM NOT DONE

mod vault {
    struct Vault {
        owner: String,
        pin: u32,
    }

    impl Vault {
        fn unlock(&self, attempt: u32) -> bool {
            attempt == self.pin
        }
    }
}

fn main() {
    let v = vault::Vault {
        owner: String::from("Ferris"),
        pin: 1234,
    };

    println!("{}'s vault", v.owner);
    println!("1111 opens it? {}", v.unlock(1111));
    println!("1234 opens it? {}", v.unlock(1234));
}
