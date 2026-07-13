// TODO: Add a withdraw method to Account that checks for sufficient balance
// TODO: Add a method is_active that returns true if balance > 0

mod bank {
    pub struct Account {
        owner: String,
        balance: u64,
    }

    impl Account {
        pub fn new(owner: &str) -> Self {
            Self {
                owner: owner.into(),
                balance: 0,
            }
        }

        pub fn deposit(&mut self, amount: u64) {
            self.balance += amount;
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }

        // TODO: implement withdraw(&mut self, amount: u64) -> Result<(), String>
        // It should return Err if amount > balance, otherwise subtract and return Ok(())

        // TODO: implement is_active(&self) -> bool that returns true if balance > 0
    }
}

fn main() {
    let mut acc = bank::Account::new("Ada");
    acc.deposit(100);
    println!("Balance: {}", acc.balance());

    match acc.withdraw(30) {
        Ok(()) => println!("Withdrew 30, new balance: {}", acc.balance()),
        Err(e) => println!("Error: {e}"),
    }

    match acc.withdraw(200) {
        Ok(()) => println!("Withdrew"),
        Err(e) => println!("Error: {e}"), // should print error
    }

    println!("Account active: {}", acc.is_active());
    // acc.balance = 999;  // ✗ field is private!
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::bank::Account;

    #[test]
    fn test_new_account_balance_zero() {
        let acc = Account::new("Bob");
        assert_eq!(acc.balance(), 0);
    }

    #[test]
    fn test_deposit() {
        let mut acc = Account::new("Bob");
        acc.deposit(50);
        assert_eq!(acc.balance(), 50);
    }

    #[test]
    fn test_withdraw_ok() {
        let mut acc = Account::new("Bob");
        acc.deposit(100);
        assert!(acc.withdraw(40).is_ok());
        assert_eq!(acc.balance(), 60);
    }

    #[test]
    fn test_withdraw_insufficient() {
        let mut acc = Account::new("Bob");
        acc.deposit(10);
        assert!(acc.withdraw(20).is_err());
        assert_eq!(acc.balance(), 10); // balance unchanged
    }

    #[test]
    fn test_is_active() {
        let mut acc = Account::new("Bob");
        assert!(!acc.is_active());
        acc.deposit(1);
        assert!(acc.is_active());
    }
}
