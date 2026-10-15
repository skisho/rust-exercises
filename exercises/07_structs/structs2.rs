// structs2.rs
//
// Add an `impl` block for `Account` with:
//   - `open(owner)`: an associated function creating an account with a
//     balance of 0
//   - `balance(&self) -> u64`
//   - `deposit(&mut self, amount)`
//   - `withdraw(&mut self, amount) -> bool`: takes the money out and
//     returns true, or returns false and changes nothing if there isn't
//     enough
//
// Think about which methods need `&self` and which need `&mut self`.

// I AM NOT DONE

struct Account {
    owner: String,
    balance: u64,
}

// TODO: impl Account { ... }

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accounts_are_empty() {
        let acc = Account::open("Ferris");
        assert_eq!(acc.owner, "Ferris");
        assert_eq!(acc.balance(), 0);
    }

    #[test]
    fn deposit_then_withdraw() {
        let mut acc = Account::open("Ferris");
        acc.deposit(100);
        acc.deposit(50);
        assert!(acc.withdraw(30));
        assert_eq!(acc.balance(), 120);
    }

    #[test]
    fn overdraw_is_refused() {
        let mut acc = Account::open("Ferris");
        acc.deposit(20);
        assert!(!acc.withdraw(21));
        assert_eq!(acc.balance(), 20);
        assert!(acc.withdraw(20));
        assert_eq!(acc.balance(), 0);
    }
}
