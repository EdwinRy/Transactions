use std::collections::{HashMap};

use crate::{transaction::{TransactionId, Transaction}, client::{ClientId, Client}};


/// Storage for transactions and client data
#[derive(Debug, PartialEq)]
pub struct Store {
    transactions: HashMap<TransactionId, Transaction>,
    clients : HashMap<ClientId, Client>,
}

impl Store {
    /// Init storage space for transactions and clients
    pub fn new() -> Store {
        Store {
            transactions: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    /// Get client account with ID, if not found, create a new account with the ID and return it
    pub fn get_or_create_client(&mut self, id: ClientId) -> &mut Client {
        self.clients.entry(id).or_insert(Client::default(id))
    }

    /// Insert transaction for storage
    pub fn save_transaction(&mut self, transaction: Transaction) {
        self.transactions.insert(transaction.id(), transaction);
    }

    /// Get transaction from storage
    pub fn get_transaction(&mut self, id: TransactionId) -> Option<&mut Transaction> {
        self.transactions.get_mut(&id)
    }

    /// Output a CSV of all customer records
    pub fn print_clients(&self) {

        // Print header
        println!("{0: <10}, {1: <10}, {2: <10}, {3: <10}, {4: <10}", 
                "client", "available", "held", "total", "locked");

        // Print values
        for (_, client) in &self.clients {
            println!("{0: <10}, {1: <10}, {2: <10}, {3: <10}, {4: <10}", 
                    client.id(), client.available(), client.held(), client.total(), client.locked()); 
        }
    }
}




#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn create_new_store() {
        let test_store = Store {
            transactions: HashMap::new(),
            clients: HashMap::new()
        };
        assert_eq!(test_store, Store::new());
    }
    
    #[test]
    fn get_or_create_client() {
        let mut test_store = Store::new();
        let new_client1 = test_store.get_or_create_client(1);
        assert_eq!(new_client1.id(), 1);
    }

    #[test]
    fn get_or_create_client_2() {
        let mut test_store = Store::new();
        test_store.clients.insert(
            5, Client::new(5, Decimal::from_str("123.4567").unwrap(), Decimal::from_str("123.4567").unwrap(), false)
        );
        let existing_client = test_store.get_or_create_client(5);
        assert_eq!(existing_client.id(), 5);
        assert_eq!(existing_client.available(), Decimal::from_str("123.4567").unwrap());
    }

}