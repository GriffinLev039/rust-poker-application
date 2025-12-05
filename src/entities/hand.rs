use std::cmp::Ordering;
use std::collections::HashMap;

use crate::entities::card::Card;
use crate::entities::hand_type::HandType;
use crate::entities::value::Value;

//Declaring "magic numbers" here
const ACE_VALUE: u32 = 14;
const FIVE_VALUE: u32 = 5;

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: Option<HandType>,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: vec![],
            hand_type: None,
        }
    }
}

impl From<Vec<Card>> for Hand {
    fn from(value: Vec<Card>) -> Self {
        Hand {
            cards: value,
            hand_type: None,
        }
    }
}

impl Hand {
    // ======================
    // COMPARISON LOGIC
    // ======================

    fn is_flush(&self) -> bool {
        if self.cards.is_empty() {
            return false;
        }
        println!("{:?}", self.cards);
        let mut suits = HashMap::from([
            (String::from("Diamond"), 0),
            (String::from("Heart"), 0),
            (String::from("Spade"), 0),
            (String::from("Club"), 0),
        ]);
        // println!("{:?}", suits);
        for card in &self.cards {
            // println!("{:?}", card.suit.to_string());
            suits
                .entry(card.suit.to_string())
                .and_modify(|count| *count += 1);
        }
        // println!("~~~");
        // println!("{:?}", suits);

        for key in suits.keys() {
            if *suits.get_key_value(key).unwrap().1 >= 5 {
                return true;
            }
        }
        return false;
    }

    fn is_straight(&mut self) -> bool {
        self.cards.sort();
        self.cards.reverse();
        let numeric_hand: Vec<u32> = self.cards.iter().map(|c| c.value.numeric_value()).collect();

        if numeric_hand[0] == ACE_VALUE && numeric_hand[1] == FIVE_VALUE {
            for i in 1..numeric_hand.len() - 1 {
                if numeric_hand[i] - numeric_hand[i + 1] != 1 {
                    return false;
                }
            }
            return true;
        }
        for i in 0..numeric_hand.len() - 1 {
            if numeric_hand[i] - numeric_hand[i + 1] != 1 {
                return false;
            }
        }
        true
    }

    fn check_pairs(&mut self) -> HandType {
        let mut card_map: HashMap<Value, u32> = HashMap::new();
        for card in &self.cards {
            card_map.insert(card.value, card_map.get(&card.value).unwrap_or(&0) + 1);
        }
        for (key, val) in &card_map {
            if *val == 4 {
                for (key2, val2) in &card_map {
                    if val2 == &1 {
                        //Finds highest card present.
                        return HandType::FourKind {
                            kind_value: *key,
                            kicker: *key2,
                        };
                    }
                }
            }
            if *val == 3 {
                for (key2, val2) in &card_map {
                    if val2 == &2 {
                        return HandType::FullHouse {
                            three_value: *key,
                            two_value: *key2,
                        };
                    }
                }
                return HandType::ThreeKind {
                    three_value: *key,
                    other_cards: card_map.clone().into_keys().filter(|c| c != key).collect(),
                }; //TODO: Fix cloning here later!
            }

            if *val == 2 {
                for (key2, val2) in &card_map {
                    if *val2 == 3 {
                        return HandType::FullHouse {
                            three_value: *key2,
                            two_value: *key,
                        };
                    } else if *val2 == 2 && key != key2 {
                        return HandType::TwoPair {
                            pair_one: *key,
                            pair_two: *key2,
                            kicker: *card_map.iter().find(|c| *c.1 == 1).unwrap().0,
                        };
                    }
                }
                return HandType::Pair {
                    pair: *key,
                    other_cards: card_map.clone().into_keys().filter(|c| c != key).collect(),
                }; //TODO: Fix cloning here!
            }
        }
        HandType::HighCard {
            cards: self.cards.iter().map(|c| c.value).collect(),
        }
    }

    pub fn determine_hand(&mut self) -> HandType {
        self.cards.sort();
        self.cards.reverse();
        let highest_card = self.cards[0];
        if self.is_flush() {
            if self.is_straight() {
                if highest_card.numeric_value() == ACE_VALUE {
                    return HandType::RoyalFlush;
                } else {
                    return HandType::StraightFlush {
                        kicker: highest_card.value,
                    };
                }
            } else {
                return HandType::Flush {
                    cards: self.cards.iter().map(|c| c.value).collect(),
                };
            }
        } else if self.is_straight() {
            return HandType::Straight {
                kicker: highest_card.value,
            };
        }
        self.check_pairs()
    }

    // ============================
    // Card Functionality
    // ============================
    pub fn draw_card(&mut self, value: Card) {
        self.cards.push(value);
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand_type.cmp(&other.hand_type))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entities::suit::Suit;


    #[test]
    fn constructor_test() {
        let mut _h: Hand = Hand::from(vec![Card::default(); 4]);
        let mut _h: Hand = Hand::default();
        let mut _h: Hand = Hand::from(vec![]);
    }
    #[test]
    fn is_straight_test() {
        let c1 = Card::new(Suit::Club, Value::Five);
        let c2 = Card::new(Suit::Spade, Value::Four);
        let c3 = Card::new(Suit::Club, Value::Three);
        let c4 = Card::new(Suit::Diamond, Value::Two);
        let c5 = Card::new(Suit::Heart, Value::Ace);
        assert!(Hand::from(vec![c1, c2, c3, c4, c5]).is_straight());

        let c6 = Card::new(Suit::Diamond, Value::Ace);
        let c7 = Card::new(Suit::Heart, Value::King);
        let c8 = Card::new(Suit::Club, Value::Queen);
        let c9 = Card::new(Suit::Spade, Value::Jack);
        let c10 = Card::new(Suit::Spade, Value::Ten);
        assert!(Hand::from(vec![c6, c7, c8, c9, c10]).is_straight());
    }

    #[test]
    fn is_flush_test() {
        let c1 = Card::new(Suit::Heart, Value::Ace);
        assert!(Hand::from(vec![c1, c1, c1, c1, c1]).is_flush());
    }

    #[test]
    fn determine_hand_test() {
        let c1 = Card::new(Suit::Spade, Value::Ace);
        let c2 = Card::new(Suit::Spade, Value::King);
        let c3 = Card::new(Suit::Spade, Value::Queen);
        let c4 = Card::new(Suit::Spade, Value::Jack);
        let c5 = Card::new(Suit::Spade, Value::Ten);
        let c6 = Card::new(Suit::Spade, Value::Nine);
        let c7 = Card::new(Suit::Diamond, Value::Nine);
        // ROYAL FLUSH
        assert_eq!(
            Hand::from(vec![c1, c2, c3, c4, c5]).determine_hand(),
            HandType::RoyalFlush
        );
        // STRAIGHT FLUSH
        assert_eq!(
            Hand::from(vec![c2, c3, c4, c5, c6]).determine_hand(),
            HandType::StraightFlush {
                kicker: Value::King
            }
        );
        // FOUR KIND
        assert_eq!(
            Hand::from(vec![c2, c2, c2, c2, c7]).determine_hand(),
            HandType::FourKind {
                kind_value: c2.value,
                kicker: c7.value
            }
        );
        // FULL HOUSE
        assert_eq!(
            Hand::from(vec![c2, c2, c2, c7, c7]).determine_hand(),
            HandType::FullHouse {
                three_value: c2.value,
                two_value: c7.value
            }
        );

        // FLUSH
        assert_eq!(
            Hand::from(vec![c1, c2, c3, c2, c4]).determine_hand(),
            HandType::Flush {
                cards: vec![c2.value, c2.value, c2.value, c2.value, c7.value]
            }
        );

        // STRAIGHT
        assert_eq!(
            Hand::from(vec![c2, c3, c4, c5, c7]).determine_hand(),
            HandType::Straight { kicker: c2.value }
        );

        // THREE KIND
        assert_eq!(
            Hand::from(vec![c2, c2, c2, c5, c7]).determine_hand(),
            HandType::ThreeKind {
                three_value: c2.value,
                other_cards: vec![c5.value, c6.value]
            }
        );
        // TWO PAIR
        assert_eq!(
            Hand::from(vec![c2, c2, c3, c3, c7]).determine_hand(),
            HandType::TwoPair {
                pair_one: c2.value,
                pair_two: c3.value,
                kicker: c6.value
            }
        );
        // PAIR
        assert_eq!(
            Hand::from(vec![c1, c2, c3, c7, c7]).determine_hand(),
            HandType::Pair {
                pair: c6.value,
                other_cards: vec![c1.value, c2.value, c3.value]
            }
        );
        // HIGH CARD
        assert_eq!(
            Hand::from(vec![c1, c2, c3, c4, c7]).determine_hand(),
            HandType::HighCard {
                cards: vec![c1.value, c2.value, c3.value, c4.value, c7.value]
            }
        );
    }
}
