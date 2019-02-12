pub trait Bank {
    fn deposit(&mut self, val: i32);
    fn withdraw(&mut self, val: i32) -> bool;
    fn balance_inquiry(self) -> i32;
}

#[derive(Copy, Clone)]
pub struct FirstBank { balance: i32 }

impl FirstBank {
    pub fn new() -> FirstBank { FirstBank { balance: 0 } }
}

impl Bank for FirstBank {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount
    }

    fn withdraw(&mut self, amount: i32) -> bool {
        if self.balance < amount {
            return false;
        } else {
            self.balance -= amount;
            return true;
        }
    }

    fn balance_inquiry(self) -> i32 {
        return self.balance;
    }
}