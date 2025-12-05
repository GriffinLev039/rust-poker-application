use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
            Suit::Spade => write!(f, "Spade"),
        }
    }
}


// impl fmt::Display for Suit {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Suit::Heart => write!(f, "♥"),
//             Suit::Diamond => write!(f, "♦"),
//             Suit::Club => write!(f, "♣"),
//             Suit::Spade => write!(f, "♠"),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use crate::entities::card::Card;
    use crate::entities::suit::Suit;
    use crate::entities::value::Value;
    use super::*;
    #[test]
    fn eq_test() {
        assert_eq!(
            Suit::Heart,
            Card::new(Suit::Heart, Value::Ace).suit
        );
        assert_eq!(Suit::Spade, Suit::Spade);
        assert_eq!(Suit::Diamond, Suit::Diamond);
        assert_eq!(Suit::Club, Suit::Club);
        assert_ne!(Suit::Heart, Suit::Diamond);
    }
    // #[test]
    // fn display_test() {
    //     assert_eq!(Suit::Spade.to_string(), "♠");
    //     assert_eq!(Suit::Heart.to_string(), "♥");
    //     assert_eq!(Suit::Diamond.to_string(), "♦");
    //     assert_eq!(Suit::Club.to_string(), "♣");
    // }
}