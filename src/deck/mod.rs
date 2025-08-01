use rand::rng;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::card::suit::Suit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for value in 1..=13 {
            for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
                cards.push(
                    Card {
                        suit: suit.clone(),
                        value: value
                    }
                );
            }
        }

        let mut new_deck = Deck { cards };
        new_deck.shuffle();
        new_deck
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
