pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn connect(self) {
        println!("Connecting to server...{}", self.address);
    }
}