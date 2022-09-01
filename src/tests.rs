#[cfg(test)]
mod tests {
    use rust_decimal::{Decimal, prelude::FromPrimitive};

    use crate::{transaction::{Transaction, TransactionKind}, store::Store, client::Client};

    #[test]
    fn sample_usage() {
        let test_transactions = vec![
            // Initialise clients with base currency
            Transaction::new(TransactionKind::Deposit, 1, 1, Decimal::from_u32(100).unwrap()),
            Transaction::new(TransactionKind::Deposit, 2, 2, Decimal::from_u32(200).unwrap()),
            Transaction::new(TransactionKind::Deposit, 3, 3, Decimal::from_u32(300).unwrap()),
            Transaction::new(TransactionKind::Deposit, 4, 4, Decimal::from_u32(400).unwrap()),
            
            // Client 1

            // withdraw an amount
            Transaction::new(TransactionKind::Withdrawal, 1, 5, Decimal::from_u32(10).unwrap()),
            // Try to withdraw too much
            Transaction::new(TransactionKind::Withdrawal, 1, 6, Decimal::from_u32(10000).unwrap()),


            // Client 2

            // Deposit an amount
            Transaction::new(TransactionKind::Deposit, 2, 7, Decimal::from_u32(200).unwrap()), 
            // Dispute just added funds
            Transaction::new(TransactionKind::Dispute, 2, 7, Decimal::from_u32(200).unwrap()), 
            // Dispute a transaction that doesn't exist
            Transaction::new(TransactionKind::Dispute, 2, 70000, Decimal::from_u32(200).unwrap()), 


            // Client 3
            
            // Deposit an amount
            Transaction::new(TransactionKind::Deposit, 3, 8, Decimal::from_u32(200).unwrap()), 
            // Dispute just added funds
            Transaction::new(TransactionKind::Dispute, 3, 8, Decimal::from_u32(200).unwrap()), 
            // Resolve the dispute
            Transaction::new(TransactionKind::Resolve, 3, 8, Decimal::from_u32(0).unwrap()), 


            // Client 4

            // Deposit an amount
            Transaction::new(TransactionKind::Deposit, 4, 9, Decimal::from_u32(200).unwrap()), 
            // Dispute just added funds
            Transaction::new(TransactionKind::Dispute, 4, 9, Decimal::from_u32(200).unwrap()), 
            // Charge back the amount from dispute
            Transaction::new(TransactionKind::Chargeback, 4, 9, Decimal::from_u32(0).unwrap()), 
            // Try to deposit more
            Transaction::new(TransactionKind::Deposit, 4, 10, Decimal::from_u32(200).unwrap()), 

        ];

        // Process all the transactions
        let mut store = Store::new();
        for mut t in test_transactions {
            t.exec(&mut store);
        }

        // What our accounts should look like
        let mut expected_client_1 = Client::new(1, Decimal::from_u32(90).unwrap(), Decimal::from_u32(0).unwrap(), false);
        let mut expected_client_2 = Client::new(2, Decimal::from_u32(200).unwrap(), Decimal::from_u32(200).unwrap(), false);
        let mut expected_client_3 = Client::new(3, Decimal::from_u32(500).unwrap(), Decimal::from_u32(0).unwrap(), false);
        let mut expected_client_4 = Client::new(4, Decimal::from_u32(400).unwrap(), Decimal::from_u32(0).unwrap(), true);

        // Test of the transaction engine changed our accounts in the way we expected
        assert_eq!(&mut expected_client_1, store.get_or_create_client(1));
        assert_eq!(&mut expected_client_2, store.get_or_create_client(2));
        assert_eq!(&mut expected_client_3, store.get_or_create_client(3));
        assert_eq!(&mut expected_client_4, store.get_or_create_client(4));
        
    }
}