use std::env;
use io::TransactionReader;
use store::Store;

mod io;
mod transaction;
mod client;
mod store;

fn main() {
    // Get path to CSV to be parsed
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument - path to transactions data CSV file")
    }
    let data_path = args[1].as_str();
    
    // Get iterator for parsing transactions iteratively
    let reader = TransactionReader::new(data_path);

    // Init storage for client accounts and transactions
    let mut store = Store::new();

    // Execute each available transaction
    for mut transaction in reader {
        transaction.exec(&mut store);
    }

    // Stdout client records in CSV format
    store.print_clients();
}


#[cfg(test)]
mod tests;

