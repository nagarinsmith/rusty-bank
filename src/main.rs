use crate::bank::Bank;
use crate::bank::FirstBank;

mod bank;

fn main() {
    let mut bank = FirstBank::new();
    bank.deposit(100);
    println!("Successful deposit");
    println!("Current balance: {}", bank.balance_inquiry());
    if bank.withdraw(130) {
        println!("Successful withdraw")
    } else {
        println!("Invalid withdraw, not enough funds!")
    }
    println!("Current balance: {}", bank.balance_inquiry());
}
