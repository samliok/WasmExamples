extern crate quick_protobuf;

use std::collections::HashMap;

type Pubkey = Vec<u8>;

struct SimpleTokenContract {
    total_supply: u64,
    name: String,
    // mapping of balances
    balances: HashMap<Pubkey, u64>,
}




// TODO: add a public SimpleTokenContract that persists in storage
// pub contract : SimpleTokenContract = ...
// then export new, mint_to, and transfer methods to host
// these methods will update `contract`. 

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
        let recipient_balance = self
            .balances
            .get_mut(recipient)
            .unwrap_or(&mut default_balance);
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
        let recipient_balance = self
            .balances
            .get_mut(recipient)
            .unwrap_or(&mut default_balance);
        *recipient_balance += amount;
        let sender_balance = self
            .balances
            .get_mut(sender)
            .unwrap_or(&mut default_balance);
        *sender_balance -= amount;
        true
    }
    // get the balance of a pubkey
    fn balance(&self, address: &Pubkey) -> u64 {
        *self.balances.get(address).unwrap_or(&mut 0)
    }
}


#[no_mangle]
pub fn get_balance() -> u64 {
    let mut a = SimpleTokenContract::new(String::from("Avalanche"), 1000);
    let alice: Pubkey = vec![0; 32];
    let bob: Pubkey = vec![1; 32];
    a.balances.insert(alice.clone(), 0);
    a.balances.insert(bob.clone(), 0);
    a.mint_to(&alice, 1000);
    a.balance(&alice)
} 


fn test() {
     // Serialized data obtained from Go
     let serialized_data: &[u8] = &[10, 5, 8, 1, 16, 2, 24, 3, 32, 4, 40, 5];

     // Create a new IntVector message
     let mut int_vector = IntVector::new();
 
     // Deserialize the message
     int_vector.merge_from_bytes(serialized_data).expect("Failed to deserialize IntVector");
 
     // Access the values
     for value in int_vector.get_values() {
         println!("Value: {}", value);
     }
}

fn main() {
    let mut a = SimpleTokenContract::new(String::from("Avalanche"), 1000);
    // quasi hash - 32 x u8 = 256 bits
    let alice: Pubkey = vec![0; 32];
    let bob: Pubkey = vec![1; 32];
    a.balances.insert(alice.clone(), 0);
    a.balances.insert(bob.clone(), 0);
    println!("Initialized token called: {}", a.name);
    println!("Total supply of: {}", a.total_supply);
    println!(
        "Trying to transfer 100 from alice to bob: {}",
        a.transfer(&alice, &bob, 100)
    );
    println!("Printing 1000 to Alice: {}", a.mint_to(&alice, 1000));
    println!("Alice balance: {}", a.balance(&alice));
    println!(
        "Trying to transfer 100 from alice to bob again: {}",
        a.transfer(&alice, &bob, 100)
    );
    get_balance();
    test();
}

// using cfg(test) keeps the tests out of the main binary
#[cfg(test)]
mod tests {
    use super::*;

    // cargo test --package token_contract --bin token_contract nocapture -- tests::test_simple_contract --exact --nocapture
    #[test]
    fn test_simple_contract() {
        let mut a = SimpleTokenContract::new("Avalanche".to_owned(), 1000);

        let alice = vec![0; 32];
        let bob = vec![1; 32];

        a.balances.insert(alice.clone(), 0);
        a.balances.insert(bob.clone(), 0);

        // no balance
        assert!(!a.transfer(&alice, &bob, 100));

        // mint
        assert!(a.mint_to(&alice, 1000));
        // verify
        assert_eq!(a.balance(&alice), 1000);

        // transfer to bob
        assert!(a.transfer(&alice, &bob, 100));
        // verify
        assert_eq!(a.balance(&bob), 100);
        assert_eq!(a.balance(&alice), 900);
    }
}

