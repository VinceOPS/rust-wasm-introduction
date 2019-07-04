use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Account {
    balance: i32,
}

#[wasm_bindgen]
impl Account {
    pub fn new(amount: i32) -> Account {
        Account { balance: amount }
    }

    /// deposit money, `amount`, added to the current `balance`
    pub fn deposit(&mut self, amount: u16) -> i32 {
        self.balance = self.balance + i32::try_from(amount).ok().unwrap();
        return self.balance;
    }

    /// withdraw money, `amount`, subtracted from the current `balance`
    pub fn withdraw(&mut self, amount: u16) -> i32 {
        self.balance = self.balance - i32::try_from(amount).ok().unwrap();
        return self.balance;
    }

    pub fn get_balance(&self) -> i32 {
        return self.balance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deposits_the_given_amount() {
        let mut acc = Account::new(500);
        assert_eq!(acc.deposit(500), 1000);
    }

    #[test]
    fn it_withdraws_the_given_amount() {
        let mut acc = Account::new(500);
        assert_eq!(acc.withdraw(500), 0);
    }

    #[test]
    fn test_negative_balance_on_withdraw() {
        let mut acc = Account::new(200);
        assert_eq!(acc.withdraw(500), -300);
    }
}
