use std::collections::HashMap;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    address: String,
    balance: u64,
}

struct TokenWallet {
    wallets: HashMap<String, Wallet>,
}

impl TokenWallet {
    fn new() -> Self {
        TokenWallet {
            wallets: HashMap::new(),
        }
    }

    fn create_wallet(&mut self, address: String) {
        let wallet = Wallet {
            address: address.clone(),
            balance: 0,
        };
        self.wallets.insert(address, wallet);
        println!("Wallet created successfully!");
    }

    fn view_balance(&self, address: &String) {
        match self.wallets.get(address) {
            Some(wallet) => println!("Address: {}, Balance: {}", wallet.address, wallet.balance),
            None => println!("Wallet not found."),
        }
    }

    fn send_tokens(&mut self, from: &String, to: &String, amount: u64) {
        // Temporary variable to hold balance to transfer
        let sender_balance = {
            if let Some(sender) = self.wallets.get_mut(from) {
                if sender.balance >= amount {
                    sender.balance -= amount;
                    Some(amount)
                } else {
                    println!("Insufficient balance.");
                    None
                }
            } else {
                println!("Sender wallet not found.");
                None
            }
        };
    
        // Transfer amount if sender's balance was reduced
        if let Some(amount) = sender_balance {
            if let Some(receiver) = self.wallets.get_mut(to) {
                receiver.balance += amount;
                println!("Transferred {} tokens from {} to {}.", amount, from, to);
            } else {
                println!("Receiver wallet not found.");
            }
        }
    }
}        



fn main() {
    let mut wallet_system = TokenWallet::new();

    loop {
        println!("1. Create Wallet\n2. View Balance\n3. Send Tokens\n4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter wallet address: ");
                io::stdout().flush().unwrap();
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Failed to read input");
                wallet_system.create_wallet(address.trim().to_string());
            }
            "2" => {
                print!("Enter wallet address: ");
                io::stdout().flush().unwrap();
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Failed to read input");
                wallet_system.view_balance(&address.trim().to_string());
            }
            "3" => {
                print!("Enter sender address: ");
                io::stdout().flush().unwrap();
                let mut sender = String::new();
                io::stdin().read_line(&mut sender).expect("Failed to read input");

                print!("Enter receiver address: ");
                io::stdout().flush().unwrap();
                let mut receiver = String::new();
                io::stdin().read_line(&mut receiver).expect("Failed to read input");

                print!("Enter amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read input");
                let amount: u64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount.");
                        continue;
                    }
                };

                wallet_system.send_tokens(&sender.trim().to_string(), &receiver.trim().to_string(), amount);
            }
            "4" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}