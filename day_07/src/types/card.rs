pub trait EnumIterator {
    fn variants() -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum Card {
    A,
    K,
    Q,
    J,
    TEN,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO
}

impl EnumIterator for Card {
    fn variants() -> Vec<Self> {
        vec![Card::A, Card::K, Card::Q, Card::J, 
            Card::TEN, Card::NINE, Card::EIGHT,
            Card:: SEVEN, Card::SIX, Card::FIVE,
            Card::FOUR, Card::THREE, Card::TWO
        ]
    }
}

pub trait Strength {
    fn strength(&self) -> u8;
}

impl Strength for Card {
    fn strength(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::TEN => 10,
            Card::NINE => 9,
            Card::EIGHT => 8,
            Card::SEVEN => 7,
            Card::SIX => 6,
            Card::FIVE => 5,
            Card::FOUR => 4,
            Card::THREE => 3,
            Card::TWO => 2,
        } 
    }
}

// From char to Card
impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::TEN,
            '9' => Card::NINE,
            '8' => Card::EIGHT,
            '7' => Card::SEVEN,
            '6' => Card::SIX,
            '5' => Card::FIVE,
            '4' => Card::FOUR,
            '3' => Card::THREE,
            '2' => Card::TWO,
            _ => panic!("Invalid character for Card enum"),
        }
    }
}

impl Into<char> for Card {
    fn into(self) -> char {
        match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::TEN => 'T',
            Card::NINE => '9',
            Card::EIGHT => '8',
            Card::SEVEN => '7',
            Card::SIX => '6',
            Card::FIVE => '5',
            Card::FOUR => '4',
            Card::THREE => '3',
            Card::TWO => '2',
        }
    }
}