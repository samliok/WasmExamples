use std::collections::HashMap;

type Pubkey = Vec<u8>;

struct SimpleTokenContract {
    total_supply: u64,
    name: String,
    // mapping of balances
    balances: HashMap<Pubkey, u64>,
}

impl SimpleTokenContract {
    fn new(name: String, total_supply: u64) -> SimpleTokenContract {
        SimpleTokenContract {
            total_supply,
            name,
            balances: HashMap::new(),
        }
    }
    fn mint_to(&mut self, recipient: &Pubkey, amount: u64) -> bool {
        // print amount to recipient
        // used for testing
        let mut default_balance = 0;
        let recipient_balance = self.balances.get_mut(recipient).unwrap_or(&mut default_balance);
        *recipient_balance += amount;
        true
    }
    fn transfer(&mut self, sender: &Pubkey, recipient: &Pubkey, amount: u64) -> bool {
       // get the senders balance
       let sender_balance = self.balances.get(sender).unwrap_or(&0).clone();
       // ensure sender has enough
       if sender_balance < amount {
        return false;
       }

       // sub from sender balance, add to recipient
       let mut default_balance = 0;
       let recipient_balance = self.balances.get_mut(recipient).unwrap_or(&mut default_balance);
       *recipient_balance += amount;
       let sender_balance = self.balances.get_mut(sender).unwrap_or(&mut default_balance);
       *sender_balance -= amount;
       true
    }
    // get the balance of a pubkey
    fn balance(&self, address: &Pubkey) -> u64 {
        *self.balances.get(address).unwrap_or(&mut 0)
    }
}


fn main () {
    let mut a = SimpleTokenContract::new(String::from("Avalanche"), 1000);
    // quasi hash - 32 x u8 = 256 bits
    let alice : Pubkey = vec![0; 32];
    let bob : Pubkey = vec![1; 32];
    a.balances.insert(alice.clone(), 0);
    a.balances.insert(bob.clone(), 0);
    println!("Initialized token called: {}", a.name);
    println!("Total supply of: {}", a.total_supply);
    println!("Trying to transfer 100 from alice to bob: {}", a.transfer(&alice, &bob, 100));
    println!("Printing 1000 to Alice: {}", a.mint_to(&alice, 1000));
    println!("Alice balance: {}", a.balance(&alice));
    println!("Trying to transfer 100 from alice to bob again: {}", a.transfer(&alice, &bob, 100));
}
