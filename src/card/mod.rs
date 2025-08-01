pub mod suit;

use suit::Suit;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub value: u8
}

impl Card {
    pub fn to_rank(&self) -> String {
        match self.value {
            1 => 'A'.to_string(),
            11 => 'J'.to_string(),
            12 => 'Q'.to_string(),
            13 => 'K'.to_string(),
            _ => self.value.to_string()
        }
    }

    pub fn to_text(&self) -> String {
        let mut text = String::from(self.to_rank());
        text.push(self.suit.symbol());

        text
    }
}
