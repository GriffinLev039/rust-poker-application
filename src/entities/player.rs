use crate::entities::{card::Card, hand::Hand};

#[derive(Debug, Clone)]
pub struct Player {
    hand: Hand,
    pub has_folded: bool,
    pub is_all_in: bool,
    pub chip_stack: u32,
    pub current_bet: u32,
    pub has_acted: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            hand: Hand::default(),
            has_folded: false,
            is_all_in: false,
            chip_stack: 2000,
            current_bet: 0,
            has_acted: false,
        }
    }
}

impl From<Hand> for Player {
    fn from(value: Hand) -> Self {
        Player {
            hand: value,
            has_folded: false,
            is_all_in: false,
            chip_stack: 0,
            current_bet: 0,
            has_acted: false,
        }
    }
}

impl From<(Hand, u32)> for Player {
    fn from(value: (Hand, u32)) -> Self {
        Player {
            hand: value.0,
            has_folded: false,
            is_all_in: false,
            chip_stack: value.1,
            current_bet: 0,
            has_acted: false,
        }
    }
}

impl Player {
    pub fn mut_is_all_in(&mut self) -> &mut bool {
        &mut self.is_all_in
    }

    pub fn get_hand(&self) -> &Hand {
        &self.hand
    }

    pub fn clear_hand(&mut self) {
        self.hand = Hand::default();
    }

    pub fn has_folded(&self) -> bool {
        self.has_folded
    }

    pub fn get_chips(&self) -> u32 {
        self.chip_stack
    }

    pub fn fold(&mut self) {
        self.has_folded = true;
    }

    pub fn make_bet(&mut self, value: u32) {
        self.chip_stack -= value;
        self.current_bet += value;
    }

    pub fn get_bet(&self) -> u32 {
        self.current_bet
    }

    pub fn draw_card(&mut self, value: Card) {
        self.hand.draw_card(value);
    }

    pub fn increase_bet_to(&mut self, value: u32) {
        println!("Users current bet is {}", self.current_bet);
        println!("Users current stack is {}", self.chip_stack);
        // The value passed to increase bet to must be greater than the users current value
        self.chip_stack -= value - self.current_bet;
        self.current_bet = value;
        println!("Users current bet AFTER raise is {}", self.current_bet);
        println!("Users current stack AFTER raise is {}", self.chip_stack);
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}
