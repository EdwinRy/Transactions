use std::env;
use io::TransactionReader;
use store::Store;

mod io;
mod transaction;
mod client;
mod store;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument - path to transactions data CSV file")
    }

    let data_path = args[1].as_str();
    let reader = TransactionReader::new(data_path);

    let mut store = Store::new();

    for mut transaction in reader {
        transaction.exec(&mut store);
    }

    store.print_clients();
}


#[cfg(test)]
mod tests;

