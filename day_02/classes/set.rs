#[derive(Debug)]
pub struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl Set {
    pub fn new(red: u8, green: u8, blue: u8) -> Set {
        Set { red, green, blue }
    }

    pub fn get_red(&self) -> u8 { self.red }
    pub fn get_green(&self) -> u8 { self.green }
    pub fn get_blue(&self) -> u8 { self.blue }
}
