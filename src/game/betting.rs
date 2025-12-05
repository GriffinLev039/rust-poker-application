use crate::entities::{player::Player, pot::Pot};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Action {
    Call,
    Check,
    Bet { value: u32 },
    Raise { value: u32 },
    Fold,
    AllIn,
}

#[derive(Clone, Debug)]
pub struct Betting {
    pub pot: Pot,
    small_blind: u32,
    big_blind: u32,
    highest_bet: u32,
    ante: u32,
}

impl Default for Betting {
    fn default() -> Self {
        Betting {
            pot: Pot::default(),
            small_blind: 10,
            big_blind: 20,
            highest_bet: 0,
            ante: 5,
        }
    }
}

impl From<(u32, u32, u32)> for Betting {
    fn from(value: (u32, u32, u32)) -> Self {
        Betting {
            pot: Pot::default(),
            small_blind: value.0,
            big_blind: value.1,
            highest_bet: 0,
            ante: value.2,
        }
    }
}

impl Betting {
    // Match case from enum or a bunch of diff functions?
    // Figure it out later :)

    pub fn get_highest_bet(&self) -> u32 {
        self.highest_bet
    }

    pub fn set_highest_bet(&mut self, value: u32) {
        self.highest_bet = value;
    }

    pub fn get_small_blind(&self) -> u32 {
        self.small_blind
    }

    pub fn get_big_blind(&self) -> u32 {
        self.big_blind
    }

    pub fn get_ante(&self) -> u32 {
        self.ante
    }

    fn set_small_blind(&mut self, value: u32) {
        self.small_blind = value;
    }

    fn set_big_blind(&mut self, value: u32) {
        self.big_blind = value;
    }

    fn get_chips(&self) -> u32 {
        self.pot.get_chips()
    }

    fn get_side_pots(&self) -> Vec<u32> {
        self.pot.get_side_pots()
    }

    // fn get_pot_differences(&self) -> Vec<u32> {
    //     self.pot.get_pot_differences()
    // }

    pub fn validate_action(&self, action: Action, player: &Player) -> Result<Action, String> {
        match action {
            // If the player has more chips than what they want to bet, the program successfully returns Ok(Bet)
            // If the player has less chips or chips equal to what they want to bet, the program returns Ok(AllIn)
            // Otherwise, returns Err
            Action::Bet { value: v } => {
                if v < player.get_chips() {
                    Ok(Action::Bet { value: v })
                } else if v <= player.get_chips() {
                    Ok(Action::AllIn)
                } else {
                    Err(String::from("Bet exceeds current chips."))
                }
            }
            // If the player more chips than the raise value, return Ok(Raise)
            // If the player has the same amount or less, return Ok(AllIn)
            // Otherwise return Err
            Action::Raise { value: v } => {
                if player.get_chips() > v && self.highest_bet < v {
                    Ok(Action::Raise { value: v })
                } else if player.get_chips() <= v && self.highest_bet < v {
                    Ok(Action::AllIn)
                } else {
                    Err(String::from("Invalid Raise"))
                }
            }

            // If the player has more chips than call, the program returns Ok(Call)
            // If the player has chips equal to or less than call, the program returns Ok(AllIn)
            Action::Call => {
                if player.get_chips() > self.get_highest_bet() {
                    return Ok(Action::Call);
                } else if player.get_chips() <= self.get_highest_bet() {
                    return Ok(Action::AllIn);
                } else {
                    return Err(String::from("This shouldn't ever be reached tbh"));
                }
            }
            //If the player has the same amount of chips bet as the highest current bet, then return Ok(Check)
            //Otherwise, returns Err
            Action::Check => {
                if self.get_highest_bet() <= player.current_bet {
                    return Ok(Action::Check);
                } else {
                    Err(String::from("You don't have enough chips bet to check."))
                }
            }

            //No real validation needed for a fold tbh
            Action::Fold => Ok(Action::Fold),

            // No real validation needed
            Action::AllIn => Ok(Action::AllIn),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_action_test() {
        let mut betting_obj = Betting::from((10, 20, 5));
        // Validate Bet
        let mut betting_player = Player::default();
        betting_player.chip_stack = 2000;
        assert_eq!(
            betting_obj
                .validate_action(Action::Bet { value: 500 }, &betting_player.clone())
                .unwrap(),
            Action::Bet { value: 500 }
        );
        // Validate Bet >> All In
        assert_eq!(
            betting_obj
                .validate_action(Action::Bet { value: 2000 }, &betting_player.clone())
                .unwrap(),
            Action::AllIn
        );
        assert!(
            betting_obj
                .validate_action(Action::Bet { value: 2005 }, &betting_player.clone())
                .is_err()
        );

        // Validate Raise
        assert_eq!(
            betting_obj
                .validate_action(Action::Raise { value: 500 }, &betting_player.clone())
                .unwrap(),
            Action::Raise { value: 500 }
        );
        // Validate Raise >> All In
        assert_eq!(
            betting_obj
                .validate_action(Action::Raise { value: 2000 }, &betting_player.clone())
                .unwrap(),
            Action::AllIn
        );
        assert_eq!(
            betting_obj
                .validate_action(Action::Raise { value: 2500 }, &betting_player.clone())
                .unwrap(),
            Action::AllIn
        );

        // Invalid Raise (value not higher than highest_bet)
        betting_obj.highest_bet = 600; // Set a highest bet
        betting_player.chip_stack = 2000; // Ensure player has enough chips to make the raise valid otherwise
        assert!(
            betting_obj
                .validate_action(Action::Raise { value: 500 }, &betting_player.clone())
                .is_err()
        );

        // Validate Call
        betting_obj.highest_bet = 500;
        assert_eq!(
            betting_obj
                .validate_action(Action::Call, &betting_player.clone())
                .unwrap(),
            Action::Call
        );
        // Validate Call >> All In
        betting_obj.highest_bet = 2000;
        assert_eq!(
            betting_obj
                .validate_action(Action::Call, &betting_player.clone())
                .unwrap(),
            Action::AllIn
        );
        betting_obj.highest_bet = 2005;
        assert_eq!(
            betting_obj
                .validate_action(Action::Call, &betting_player.clone())
                .unwrap(),
            Action::AllIn
        );
        // Validate Check
        betting_obj.highest_bet = 500;
        betting_player.current_bet = 500;
        assert_eq!(
            betting_obj
                .validate_action(Action::Check, &betting_player.clone())
                .unwrap(),
            Action::Check
        );
        betting_player.current_bet = 250;

        // Invalid Check
        assert!(
            betting_obj
                .validate_action(Action::Check, &betting_player.clone())
                .is_err()
        );
        // Validate Fold
        assert_eq!(
            betting_obj
                .validate_action(Action::Fold, &betting_player.clone())
                .unwrap(),
            Action::Fold
        );
        // Validate AllIn
        assert_eq!(
            betting_obj
                .validate_action(Action::AllIn, &betting_player.clone())
                .unwrap(),
            Action::AllIn
        );
    }
}
