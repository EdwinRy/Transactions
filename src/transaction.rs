
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{client::ClientId, store::Store};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(tag="type")]
#[serde(rename_all="lowercase")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub type TransactionId = u32;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Transaction {
    #[serde(alias="type")]
    #[serde(flatten)]
    kind: TransactionKind,
    #[serde(rename="client")]
    client_id: ClientId,
    #[serde(rename="tx")]
    transaction_id: TransactionId,
    amount: Decimal,
    #[serde(skip_deserializing)]
    success: bool,
}


impl Transaction {
    pub fn id(&self) -> TransactionId { self.transaction_id }

    // TODO: this is used by the tests and would most definitely be used in a larger codebase
    #[allow(dead_code)]
    pub fn new(kind: TransactionKind, client_id: ClientId, transaction_id: TransactionId, amount: Decimal) -> Transaction {
        Transaction{ kind, client_id, transaction_id, amount, success: true }
    }

    pub fn exec(&mut self, store: &mut Store) {
        match self.kind {
            TransactionKind::Deposit => self.deposit(store),
            TransactionKind::Withdrawal => self.withdraw(store),
            TransactionKind::Dispute => self.dispute(store),
            TransactionKind::Resolve => self.resolve(store),
            TransactionKind::Chargeback => self.chargeback(store),
        }
    }

    fn deposit(&mut self, store: &mut Store) {
        let client = store.get_or_create_client(self.client_id);
        if client.locked() { return ;}
        client.deposit(self.amount);
        self.success = true;
        store.save_transaction(self.clone());
    }

    fn withdraw(&mut self, store: &mut Store) {
        let client = store.get_or_create_client(self.client_id);
        if client.locked() { return ;}
        self.success = client.withdraw(self.amount);
        store.save_transaction(self.clone());
    }

    fn dispute(&mut self, store: &mut Store) {
        let disputed_transaction = store.get_transaction(self.id()).cloned();
        let client = store.get_or_create_client(self.client_id); 
        if client.locked() { return ;}
        match disputed_transaction {
            None => return ,
            Some(transaction) => {
                if !client.dispute(transaction.amount) {
                    // TODO: Handle dispute being over an amount greater than is present on client's account
                    todo!();
                }
            }
        }
    }

    fn resolve(&mut self, store: &mut Store) {
        let disputed_transaction = store.get_transaction(self.id()).cloned();
        let client = store.get_or_create_client(self.client_id); 
        if client.locked() { return ;}
        match disputed_transaction {
            None => return ,
            Some(transaction) => {
                if !client.resolve(transaction.amount) {
                    // TODO: Handle resolution where there isn't enough funds in held to transfer to available
                    todo!();
                }
            }
        }
    }
    
    fn chargeback(&mut self, store: &mut Store) {
        let disputed_transaction = store.get_transaction(self.id()).cloned();
        let client = store.get_or_create_client(self.client_id); 
        if client.locked() { return ;}
        match disputed_transaction {
            None => return ,
            Some(transaction) => {
                if !client.chargeback(transaction.amount) {
                    // TODO: Handle when charge back can't take place due to insufficient held funds
                    todo!();
                }
            }
        }
    }

}




#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::{Decimal, prelude::FromPrimitive};

    use crate::store::Store;

    use super::{Transaction, TransactionKind};

    #[test]
    fn new() {
        let test_transaction = Transaction { 
            kind: TransactionKind::Deposit, client_id: 1, transaction_id: 1, 
            amount: Decimal::from_u32(100).unwrap(), success: true };
        assert_eq!(test_transaction, Transaction::new(TransactionKind::Deposit, 1, 1, Decimal::from_u32(100).unwrap()));
    }

    #[test]
    fn deposit() {
        let mut store = Store::new();
        
        let test_id = 1;
        let mut deposit_transaction = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_str("100").unwrap(), success: true};

        deposit_transaction.exec(&mut store);
        
        let client = store.get_or_create_client(test_id);
        assert_eq!(client.id(), 1);
        assert_eq!(client.available(), Decimal::from_u32(100).unwrap());
        assert_eq!(client.total(), Decimal::from_u32(100).unwrap());
    }

    #[test]
    fn withdraw() {
        let mut store = Store::new();
        
        let test_id = 1;
        // Deposit 100 onto account
        let mut deposit_transaction = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_str("100").unwrap(), success: true};

        deposit_transaction.exec(&mut store);

        // Withdraw 25 from the account
        let mut withdrawal_transaction = Transaction {
            kind: TransactionKind::Withdrawal, client_id: test_id, transaction_id: 2, 
            amount: Decimal::from_str("25").unwrap(), success: true};

        withdrawal_transaction.exec(&mut store);
        
        let client = store.get_or_create_client(test_id);
        assert_eq!(client.id(), 1);
        assert_eq!(client.available(), Decimal::from_u32(75).unwrap());
        assert_eq!(client.total(), Decimal::from_u32(75).unwrap());
    }


    #[test]
    fn dispute() {
        let mut store = Store::new();
        
        let test_id = 1;

        // Add 100 onto account
        let mut deposit_transaction = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_str("100").unwrap(), success: true};

        deposit_transaction.exec(&mut store);
       
        // Add 50 onto account
        let mut deposit_transaction2 = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 2, 
            amount: Decimal::from_str("50").unwrap(), success: true};

            deposit_transaction2.exec(&mut store);

        // Dispute the 100 deposit
        let mut dispute_transaction = Transaction {
            kind: TransactionKind::Dispute, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_u32(0).unwrap(), success: true};

        dispute_transaction.exec(&mut store);
        
        let client = store.get_or_create_client(test_id);
        assert_eq!(client.id(), 1);
        assert_eq!(client.available(), Decimal::from_u32(50).unwrap());
        assert_eq!(client.held(), Decimal::from_u32(100).unwrap());
        assert_eq!(client.total(), Decimal::from_u32(150).unwrap());
    }

    #[test]
    fn resolve() {
        let mut store = Store::new();
        
        let test_id = 1;

        // Add 100 onto account
        let mut deposit_transaction = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_str("100").unwrap(), success: true};

        deposit_transaction.exec(&mut store);
       
        // Add 50 onto account
        let mut deposit_transaction2 = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 2, 
            amount: Decimal::from_str("50").unwrap(), success: true};

            deposit_transaction2.exec(&mut store);

        // Dispute the 100 deposit
        let mut dispute_transaction = Transaction {
            kind: TransactionKind::Dispute, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_u32(0).unwrap(), success: true};
       
        dispute_transaction.exec(&mut store);
       
        // Resolve the 100 deposit dispute
        let mut resolve_transaction = Transaction {
            kind: TransactionKind::Resolve, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_u32(0).unwrap(), success: true};

        resolve_transaction.exec(&mut store);
        
        let client = store.get_or_create_client(test_id);
        assert_eq!(client.id(), 1);
        assert_eq!(client.available(), Decimal::from_u32(150).unwrap());
        assert_eq!(client.held(), Decimal::from_u32(0).unwrap());
        assert_eq!(client.total(), Decimal::from_u32(150).unwrap());
    }

    #[test]
    fn chargeback() {
        let mut store = Store::new();
        
        let test_id = 1;

        // Add 100 onto account
        let mut deposit_transaction = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_str("100").unwrap(), success: true};

        deposit_transaction.exec(&mut store);
       
        // Add 50 onto account
        let mut deposit_transaction2 = Transaction {
            kind: TransactionKind::Deposit, client_id: test_id, transaction_id: 2, 
            amount: Decimal::from_str("50").unwrap(), success: true};

            deposit_transaction2.exec(&mut store);

        // Dispute the 100 deposit
        let mut dispute_transaction = Transaction {
            kind: TransactionKind::Dispute, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_u32(0).unwrap(), success: true};
       
        dispute_transaction.exec(&mut store);
       
        // Charge back the 100 deposit dispute
        let mut chargeback_transaction = Transaction {
            kind: TransactionKind::Chargeback, client_id: test_id, transaction_id: 1, 
            amount: Decimal::from_u32(0).unwrap(), success: true};

        chargeback_transaction.exec(&mut store);
        
        let client = store.get_or_create_client(test_id);
        assert_eq!(client.id(), 1);
        assert_eq!(client.available(), Decimal::from_u32(50).unwrap());
        assert_eq!(client.held(), Decimal::from_u32(0).unwrap());
        assert_eq!(client.total(), Decimal::from_u32(50).unwrap());
        assert_eq!(client.locked(), true);
    }


}