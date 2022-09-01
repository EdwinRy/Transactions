use rust_decimal::Decimal;

pub type ClientId = u16;

#[derive(Debug, PartialEq)]
pub struct Client {
    id: ClientId,
    available: Decimal,
    held: Decimal,
    locked: bool,
}


impl Client {
    
    // GETTERS
    pub fn id(&self) -> ClientId { self.id }
    pub fn available(&self) -> Decimal { self.available }
    pub fn held(&self) -> Decimal { self.held }
    pub fn locked(&self) -> bool { self.locked }
    pub fn total(&self) -> Decimal { self.available + self.held }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) -> bool {
        if self.available < amount { return false; }
        else {
            self.available -= amount;
            return true;
        }
    }

    pub fn dispute(&mut self, amount: Decimal) -> bool {
        if self.available < amount { return false; }
        else {
            self.available -= amount;
            self.held += amount;
            return true;
        }
    }

    pub fn resolve(&mut self, amount: Decimal) -> bool {
        if self.held < amount { return false; }
        else {
            self.held -= amount;
            self.available += amount;
            return true;
        }
    }
    
    pub fn chargeback(&mut self, amount: Decimal) -> bool {
        if self.held < amount { return false; }
        else {
            self.locked = true;
            self.held -= amount;
            return true;
        }
    }

    // TODO: this is used by the tests and would most definitely be used in a larger codebase
    #[allow(dead_code)]
    pub fn new(id: ClientId, available: Decimal, held: Decimal, locked: bool) -> Client {
        Client { id, available, held, locked }
    }

    pub fn default(id: ClientId) -> Client {
        Client { 
            id, 
            available: Decimal::new(0, 4), 
            held: Decimal::new(0, 4), 
            locked: false,
        }
    }
}




#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn create_default_client() {
        let test_id = 1;
        let default_client = Client{
            id: test_id, available: Decimal::new(0, 4), 
            held: Decimal::new(0, 4), locked: false};
        assert_eq!(default_client, Client::default(test_id));
    }

    #[test]
    fn create_custom_client() {
        let test_id = 1;

        let test_client = Client{
            id: test_id, available: Decimal::new(534543654, 4), 
            held: Decimal::new(543324534543, 4), locked: false};

        let custom_client = Client::new(
            test_id, Decimal::new(534543654, 4), 
            Decimal::new(543324534543, 4), false);
            
        assert_eq!(test_client, custom_client); 
    }

    #[test]
    fn deposit() {
        let mut test_client = Client::new(
            1, Decimal::new(0, 4), 
            Decimal::new(0, 4), false);

        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        assert_eq!(test_client.available.to_string(), "125.2563");
        
        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        assert_eq!(test_client.available.to_string(), "250.5126");
    }
   
    #[test]
    fn withdraw() {
        let mut test_client = Client::new(
            1, Decimal::new(0, 4), 
            Decimal::new(0, 4), false);

        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        test_client.withdraw(Decimal::from_str("100").unwrap());

        assert_eq!(test_client.available.to_string(), "150.5126");
    }
   
    #[test]
    fn dispute() {
        let mut test_client = Client::new(
            1, Decimal::new(0, 4), 
            Decimal::new(0, 4), false);

        test_client.deposit(Decimal::from_str("225.2563").unwrap());
        test_client.dispute(Decimal::from_str("100").unwrap());

        assert_eq!(test_client.available.to_string(), "125.2563");
        assert_eq!(test_client.held.to_string(), "100");
    }
   
    #[test]
    fn resolve() {
        let mut test_client = Client::new(
            1, Decimal::new(0, 4), 
            Decimal::new(0, 4), false);

        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        test_client.dispute(Decimal::from_str("100").unwrap());
        test_client.resolve(Decimal::from_str("50").unwrap());

        assert_eq!(test_client.available.to_string(), "75.2563");
        assert_eq!(test_client.held.to_string(), "50");
    }

    #[test]
    fn chargeback() {
        let mut test_client = Client::new(
            1, Decimal::new(0, 4), 
            Decimal::new(0, 4), false);

        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        test_client.dispute(Decimal::from_str("100").unwrap());
        test_client.chargeback(Decimal::from_str("50").unwrap());

        assert_eq!(test_client.available.to_string(), "25.2563");
        assert_eq!(test_client.held.to_string(), "50");
    }

    #[test]
    fn properties() {
        let mut test_client = Client::new(
            1, Decimal::new(0, 4), 
            Decimal::new(0, 4), false);

        test_client.deposit(Decimal::from_str("125.2563").unwrap());
        test_client.dispute(Decimal::from_str("100").unwrap());
        test_client.chargeback(Decimal::from_str("50").unwrap());

        assert_eq!(test_client.available().to_string(), "25.2563");
        assert_eq!(test_client.held().to_string(), "50"); 
        assert_eq!(test_client.total().to_string(), "75.2563"); 
        assert_eq!(test_client.id().to_string(), "1"); 
    }

}