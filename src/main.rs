use crate::bank::Bank;
use crate::bank::FirstBank;

mod bank;

fn main() {
    let mut bank = FirstBank::new();
    bank.deposit(100);
    println!("Current balance: {}", bank.balance_inquiry());
    bank.withdraw(30);
    println!("Current balance: {}", bank.balance_inquiry());
}
