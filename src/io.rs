use crate::transaction::{Transaction};
use csv::{Reader, DeserializeRecordsIntoIter};
use std::fs::File;


pub struct TransactionReader<'a> {
    /// Path to the CSV file to read
    path: &'a str,
    /// CSV Iterator for fetching rows to be parsed
    iter: DeserializeRecordsIntoIter<File, Transaction>,
}


impl TransactionReader<'_> {
    /// Create a reader object to store the transaction record iterator created for given csv file path
    pub fn new(path: &str) -> TransactionReader<'_> {
        let reader = Reader::from_path(path)
            .unwrap_or_else( |_err| panic!("Couldn't find file: {}", path)); 

        let iter: DeserializeRecordsIntoIter<File, Transaction> = reader.into_deserialize();
        TransactionReader { path, iter }
    }
}


impl Iterator for TransactionReader<'_> {    
    type Item = Transaction;

    /// Fetch the next transaction record from the CSV
    fn next(&mut self) -> Option<Transaction> {
        // Get the next Transaction record from the reader
        match self.iter.next() {
            None => None,
            Option::Some(result) => {
                let transaction = result
                    .unwrap_or_else(|err| panic!("Couldn't parse transaction from file ({}): {}", self.path, err));
                return Some(transaction);
            },
        }
    }
}
