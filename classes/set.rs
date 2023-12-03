// set.rs

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
}
