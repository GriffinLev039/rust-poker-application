use crate::entities::{card::Card, suit::Suit, value::Value};
use rand::rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();

        for &suit in &[Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond] {
            for &value in &[
                Value::Ace,
                Value::King,
                Value::Queen,
                Value::Jack,
                Value::Ten,
                Value::Nine,
                Value::Eight,
                Value::Seven,
                Value::Six,
                Value::Five,
                Value::Four,
                Value::Three,
                Value::Two,
            ] {
                cards.push(Card::new(suit, value));
            }
        }
        cards.shuffle(&mut rng());
        Deck { cards }
    }
    // Need a way to handle empty deck just to be safe.
    // LOW TODO: Need to handle empty deck. Not urgent since deck should be shuffled every round.
    pub fn deal_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn reset_deck(&mut self) {
        let mut cards: Vec<Card> = Vec::new();

        for &suit in &[Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond] {
            for &value in &[
                Value::Ace,
                Value::King,
                Value::Queen,
                Value::Jack,
                Value::Ten,
                Value::Nine,
                Value::Eight,
                Value::Seven,
                Value::Six,
                Value::Five,
                Value::Four,
                Value::Three,
                Value::Two,
            ] {
                cards.push(Card::new(suit, value));
            }
        }
        cards.shuffle(&mut rng());
        self.cards = cards;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deck_test() {
        let deck = Deck::new();
        println!("{:?}", deck);
    }
}
