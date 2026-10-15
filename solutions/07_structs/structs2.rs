// structs2.rs - solution
//
// `open` has no `self`, so it's called as `Account::open(...)`. Reading
// the balance only needs `&self`; changing it needs `&mut self`, which is
// why the tests must declare `let mut acc`. Using a `u64` and checking
// before subtracting means the balance can never go negative.

struct Account {
    owner: String,
    balance: u64,
}

impl Account {
    fn open(owner: &str) -> Self {
        Account {
            owner: owner.to_string(),
            balance: 0,
        }
    }

    fn balance(&self) -> u64 {
        self.balance
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        true
    }
}

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
