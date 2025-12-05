use std::cmp::Ordering;

use crate::{
    entities::{card::Card, deck::Deck, hand_type::HandType, player::Player},
    game::betting::{Action, Betting},
};

pub mod betting;
mod poker_game;
mod river;

#[derive(Clone, Debug)]
pub struct Game {
    pub all_players: Vec<Player>,
    player_reference_vector: Vec<usize>,
    table_pos: usize,
    dealer_pos: usize,
    pub betting_handler: Betting,
    river: Vec<Card>,
    pub deck: Deck,
    pub has_gone_around: bool,
}

// I want to create an array that essentially keeps a reference to all
// current players starting from the dealer. If a player folds, they
// get removed from this list.
// If a player goes all in, they need some special consideration here
impl Game {
    pub fn create_player_reference_vec(&mut self) {
        self.player_reference_vector.clear();
        if self.all_players.len() == 0 {
            return;
        }
        for num in 0..self.all_players.len() {
            let player_index = (self.dealer_pos + num) % self.all_players.len();
            if !self.all_players[player_index].has_folded
                && !self.all_players[player_index].is_all_in
            {
                self.player_reference_vector.push(player_index);
            }
        }
    }

    pub fn get_current_players(&mut self) -> &Vec<usize> {
        self.create_player_reference_vec();
        &self.player_reference_vector
    }

    pub fn get_player(&self, index: usize) -> &Player {
        &self.all_players[index]
    }

    pub fn distribute_winnings(&mut self) {
        let mut winning_player_indexes = self.get_winning_players().clone();
        // println!("Winning Players: {:?}", winning_player_indexes);
        let mut winning_player_bets: Vec<u32> = winning_player_indexes
            .iter()
            .map(|index| self.all_players[*index].current_bet)
            .collect();
        let first = winning_player_bets[0];
        if winning_player_indexes.len() > 1
            && !winning_player_bets.iter().all(|&item| item == first)
        {
            // This only applies when multiple people win AND have different values they won with.

            // sorted_winning_player_indexes is sorted by max bet value decreasingly.
            winning_player_indexes.sort_by(|b, a| {
                self.all_players[*a]
                    .current_bet
                    .cmp(&self.all_players[*b].current_bet)
            });

            let mut pot_array: Vec<u32> = Vec::new();
            let mut nested_player_indexes: Vec<Vec<u32>> = Vec::new();
            let mut removed_players_indexes: Vec<u32> = Vec::new();
            let mut previous_bet: u32 = 0;
            while winning_player_bets.len() > 0 {
                let amt = winning_player_bets[0];
                if amt == previous_bet {
                    winning_player_bets.remove(0);
                    continue;
                }
                for i in 0..winning_player_indexes.len() {
                    if amt == self.all_players[winning_player_indexes[i]].current_bet
                        && !removed_players_indexes.contains(&(winning_player_indexes[i] as u32))
                    {
                        removed_players_indexes.push(winning_player_indexes[i] as u32)
                    }
                }
                let num_of_players_invested: u32 = self
                    .all_players
                    .iter()
                    .filter(|player| {
                        player.current_bet >= winning_player_bets[0] && !player.has_folded
                    })
                    .count() as u32;
                pot_array.push((winning_player_bets[0] - previous_bet) * num_of_players_invested);
                previous_bet = winning_player_bets[0];
                winning_player_bets.remove(0);
                nested_player_indexes.push(removed_players_indexes.clone());
            }
            for i in 0..pot_array.len() {
                for player_index in &nested_player_indexes[i] {
                    self.all_players[*player_index as usize].chip_stack +=
                        pot_array[i] / nested_player_indexes[i].len() as u32;
                }
            }
        } else {
            let mut difference_from_max = 0;
            let bet_diff_map: Vec<isize> = self
                .all_players
                .iter()
                .map(|p| p.current_bet as isize - self.betting_handler.get_highest_bet() as isize)
                .collect();
            // Iterates to sum all negative numbers
            for num in 0..bet_diff_map.len() {
                let val = bet_diff_map[num];
                if val > 0 {
                    self.all_players[num].chip_stack += val as u32;
                } else if val < 0 {
                    // Create a running total of chips to be subtracted from the players winnings
                    difference_from_max += -1 * val;
                }
                // If val == 0, then do nothing!
            }
            //Since players cannot win more than their bet amt * the num of players, we can just do
            // that minus the difference in peoples bets when the bet lower, and then for all positive nums just return excess.
            let winner_earnings = self.all_players.len() as isize
                * self.betting_handler.get_highest_bet() as isize
                - difference_from_max;
            // Winnings assigned
            self.all_players[winning_player_indexes[0]].chip_stack += winner_earnings as u32;
        }

        self.clear_bets();
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            all_players: vec![Player::default(); 5],
            player_reference_vector: vec![],
            table_pos: 0,
            dealer_pos: 0,
            betting_handler: Betting::default(),
            river: vec![],
            deck: Deck::new(),
            has_gone_around: false,
        }
    }
}

impl From<Vec<Player>> for Game {
    fn from(value: Vec<Player>) -> Self {
        Game {
            all_players: value,
            player_reference_vector: vec![],
            table_pos: 0,
            dealer_pos: 0,
            betting_handler: Betting::default(),
            river: vec![],
            deck: Deck::new(),
            has_gone_around: false,
        }
    }
}

// Args: Players, small blind, big blind, ante
impl From<(Vec<Player>, u32, u32, u32)> for Game {
    fn from(value: (Vec<Player>, u32, u32, u32)) -> Self {
        Game {
            all_players: value.0,
            player_reference_vector: vec![],
            table_pos: 0,
            dealer_pos: 0,
            betting_handler: Betting::from((value.1, value.2, value.3)),
            river: vec![],
            deck: Deck::new(),
            has_gone_around: false,
        }
    }
}

impl Game {
    // =====================
    // RIVER
    // =====================
    // Getter (whole hand)
    pub fn get_river(&self) -> Vec<Card> {
        // println!("{:?}", self.river.clone().len());
        self.river.clone()
    }

    // Getter (latest card)
    pub fn get_placed_card(&self) -> Option<Card> {
        if self.river.len() == 0 {
            None
        } else {
            Some(self.river[self.river.len() - 1])
        }
    }

    // Reset river
    pub fn reset_river(&mut self) {
        self.river = vec![];
    }

    // Draw a card
    // Progress River
    //If empty, put initial cards down
    //If initial cards down, one more
    //If initial cards down, one more
    //If river full, do nothing? Err?
    //
    pub fn progress_river(&mut self) {
        // println!("#####Deck cards quantity: {}", self.deck.cards.len());
        let river_len = self.river.len();
        if river_len == 0 {
            // flop
            self.deck.deal_card(); //BURNING CARD
            self.river.push(self.deck.deal_card());
            self.river.push(self.deck.deal_card());
            self.river.push(self.deck.deal_card());
        } else if river_len == 3 {
            // turn
            self.deck.deal_card(); //BURNING CARD
            self.river.push(self.deck.deal_card());
        } else {
            // river
            self.deck.deal_card(); //BURNING CARD
            self.river.push(self.deck.deal_card());
        }
    }

    // =======================
    // TABLE MANAGEMENT
    // =======================

    // Get current dealer
    pub fn get_current_dealer(&self) -> usize {
        self.dealer_pos
    }

    pub fn get_table_pos(&self) -> usize {
        self.table_pos
    }

    // set current_dealer to next dealer
    pub fn next_dealer(&mut self) {
        self.dealer_pos += 1;
        self.dealer_pos = self.dealer_pos % self.all_players.len();
    }

    // Deal cards
    pub fn deal_cards(&mut self) {
        for p in &mut self.all_players {
            p.draw_card(self.deck.deal_card());
            p.draw_card(self.deck.deal_card());
        }
    }

    // Get next player
    // Returns Result<u32, String>
    // If player returned, that player goes next
    // Otherwise if err returned, progresses on to next round.
    // Performs a check to see if all current players have
    // Bets equal to the current highest bet, are folded, or are ALL IN.
    // If this check fails, returns Err
    // If this check succeeds, returns Ok(Player) where player is the next player
    // Loops around table by:  current player num + 1 MOD num of players.
    // Updated to return index of player.
    pub fn go_next_player(&mut self) -> Result<u32, String> {
        self.create_player_reference_vec();
        let players_have_acted = self
            .player_reference_vector
            .iter()
            .all(|index| self.all_players[*index].has_acted);
        let all_bets_valid = self.player_reference_vector.iter().all(|index| {
            self.all_players[*index].current_bet == self.betting_handler.get_highest_bet()
                || self.all_players[*index].is_all_in
        });

        if players_have_acted && all_bets_valid {
            self.reset_has_acted();
            Err(String::from("Move on."))
        } else {
            self.table_pos = (self.table_pos + 1) % self.player_reference_vector.len();
            Ok(self.player_reference_vector[self.table_pos] as u32)
        }
    }

    pub fn get_next_player(&mut self) -> Result<u32, String> {
        self.create_player_reference_vec();
        if (self.player_reference_vector.len() <= 1) {
            return Err(String::from("All other players probs folded."));
        }
        let players_have_acted = self
            .player_reference_vector
            .iter()
            .all(|index| self.all_players[*index].has_acted);
        let all_bets_valid = self.player_reference_vector.iter().all(|index| {
            self.all_players[*index].current_bet == self.betting_handler.get_highest_bet()
                || self.all_players[*index].is_all_in
        });

        if players_have_acted && all_bets_valid {
            Err(String::from("Move on."))
        } else {
            // self.table_pos = (self.table_pos + 1) % self.player_reference_vector.len();
            // println!(
            //     "get_next_player -> Ok {:?}",
            //     self.all_players[self.player_reference_vector[self.table_pos]]
            // );
            Ok(self.player_reference_vector[self.table_pos] as u32)
        }
    }

    /* Old copy of the function because I don't have this on Git yet for some reason.
    *
    * pub fn go_next_player(&mut self) -> Result<&Player, String> {
        self.create_player_reference_vec();
        for index in &self.player_reference_vector {
            if self.all_players[*index].get_bet() == self.betting_handler.get_highest_bet() {
                //TODO: NOT ALL IN CHECK NEEDED HERE
                return Err(String::from(
                    "All players are betting equally, folded, or all in.",
                ));
            } else if self.table_pos == todo!() {
                //TODO!!!!!
                return Err(String::from("At the end of current table cycle?"));
            } else {
                self.table_pos = (self.table_pos + 1) % self.player_reference_vector.len();
                return Ok(&self.all_players[self.table_pos]);
            }
        }
        Err(String::from("Likely empty vector of players!"))
    }
    *
    */

    // Get big blind player fn
    pub fn get_big_blind_player(&self) -> usize {
        return (self.dealer_pos + 2) % self.all_players.len();
    }
    // Get small blind player fn
    pub fn get_small_blind_player(&self) -> usize {
        return (self.dealer_pos + 1) % self.all_players.len();
    }
    // Get ante players fn
    pub fn get_ante_players(&self) -> Vec<usize> {
        let len = self.all_players.len();
        let mut return_vec: Vec<usize> = vec![];
        let small_blind_pos = (self.dealer_pos + 2) % len;
        let big_blind_pos = (self.dealer_pos + 1) % len;
        for i in 0..len {
            if i == small_blind_pos || i == big_blind_pos {
                continue;
            }
            return_vec.push(i);
        }
        return_vec
    }

    // =================================
    // POT(S), BLINDS, ANTE, AND BETTING
    // =================================

    // Function to collect initial chips (Ante, blinds)
    pub fn collect_ante_blind(&mut self) {
        let big_blind_pos: usize = self.get_big_blind_player();
        let small_blind_pos: usize = self.get_small_blind_player();
        // Collect ante from players
        // Collect small blind
        // Collect big blind
        // If any of the above bring a player to or "below" 0 chips, mark that player as "all-in"
        // TODO: Add safety stuff for ensuring when blind/ante hits if player chips less than ante/blind its handled properly.
        self.all_players[big_blind_pos].increase_bet_to(self.betting_handler.get_big_blind());
        self.all_players[small_blind_pos].increase_bet_to(self.betting_handler.get_small_blind());
        for player_pos in self.get_ante_players() {
            self.all_players[player_pos].increase_bet_to(self.betting_handler.get_ante());
        }
        self.betting_handler
            .set_highest_bet(self.betting_handler.get_big_blind());
    }

    // Function to modify player based on action
    // Returns Result<Action> based on success
    pub fn execute_player_action(
        &mut self,
        index: usize,
        action: Action,
    ) -> Result<Action, String> {
        let player = &mut self.all_players[index];
        let validation_result = self
            .betting_handler
            .validate_action(action, player)
            .unwrap(); //Need to handle Err here

        if validation_result == action {
            match action {
                Action::AllIn => {
                    *player.mut_is_all_in() = true;
                    player.increase_bet_to(player.get_chips());
                    if (self.betting_handler.get_highest_bet() < player.get_bet()) {
                        self.betting_handler.set_highest_bet(player.get_bet());
                    }
                    player.has_acted = true;
                    player.is_all_in = true;
                    return Ok(Action::AllIn);
                }
                Action::Bet { value: v } => {
                    player.make_bet(v);
                    // Shouldn't have to check this since it's a bet. Same goes for a raise.
                    if (self.betting_handler.get_highest_bet() < player.get_bet()) {
                        self.betting_handler.set_highest_bet(player.get_bet());
                    }
                    player.has_acted = true;
                    return Ok(Action::Bet { value: v });
                }
                Action::Call => {
                    player.increase_bet_to(self.betting_handler.get_highest_bet());
                    player.has_acted = true;
                    return Ok(Action::Call);
                }
                Action::Check => {
                    // No action taken for checking.
                    player.has_acted = true;
                    return Ok(Action::Check);
                }
                Action::Fold => {
                    player.fold();
                    player.has_acted = true;
                    return Ok(Action::Fold);
                }
                Action::Raise { value: v } => {
                    // Shouldn't have to check this to set highest bet.
                    player.increase_bet_to(v);
                    player.has_acted = true;
                    self.betting_handler.set_highest_bet(player.get_bet());
                    return Ok(Action::Raise { value: v });
                }
            }
        } else if validation_result == Action::AllIn {
            player.increase_bet_to(player.chip_stack);
            if (self.betting_handler.get_highest_bet() < player.get_bet()) {
                self.betting_handler.set_highest_bet(player.get_bet());
            }
            player.has_acted = true;
            player.is_all_in = true;
            return Ok(Action::AllIn);
        } else {
            return Err(String::from("Invalid option"));
        }
    }

    // Returns the Ok(Action) if the player action is valid.
    // If player action is invalid but has a valid alternative, returns that.
    //E.g. If you try to raise but raise equal to or more than what you have, it returns OK(AllIn) instead of Ok(Raise)
    pub fn validate_player_action(&self, action: Action, player: Player) -> Result<Action, String> {
        self.betting_handler.validate_action(action, &player)
    }

    pub fn clear_bets(&mut self) {
        self.all_players
            .iter_mut()
            .for_each(|player: &mut Player| player.current_bet = 0);
    }

    pub fn clear_hands(&mut self) {
        self.all_players
            .iter_mut()
            .for_each(|player: &mut Player| player.clear_hand());
    }

    pub fn reset_has_acted(&mut self) {
        self.all_players
            .iter_mut()
            .for_each(|player| player.has_acted = false);
    }

    pub fn get_winning_players(&mut self) -> Vec<usize> {
        let active_players: Vec<usize> = self
            .all_players
            .iter()
            .filter(|player| !player.has_folded)
            .enumerate()
            .map(|(index, _player)| index)
            .collect();

        if active_players.is_empty() {
            return vec![];
        }
        let mut best_hand_type = self.all_players[active_players[0]]
            .get_hand()
            .clone()
            .determine_hand();
        let mut winning_players: Vec<usize> = vec![];
        for index in active_players {
            // println!("#################");
            let new_hand_type = self.all_players[index].get_hand().clone().determine_hand();
            match new_hand_type.cmp(&best_hand_type) {
                Ordering::Less => {
                    continue;
                }
                Ordering::Equal => {
                    winning_players.push(index);
                }
                Ordering::Greater => {
                    winning_players = vec![index];
                    best_hand_type = new_hand_type;
                }
            }
        }
        return winning_players;
    }

    pub fn get_first_player(&self) -> usize {
        self.player_reference_vector
            [self.get_current_dealer() - 1 % self.player_reference_vector.len()]
    }
}

// TODO: Add updates to betting handler states
// TODO: Fix go_next_player
// TODO: Finish execute_player_action

/*
TODO:
 - Fix go_next_player logic
    - Ensure game stops when encountering player who raised last OR if all players called/checked.
    - Ensure player and error are returned in their respective spots
*/

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_river() {
        let mut game = Game::default();
        assert_eq!(game.river.len(), 0);
        game.progress_river();
        assert_eq!(game.river.len(), 3);
        game.progress_river();
        assert_eq!(game.river.len(), 4);
        game.progress_river();
        assert_eq!(game.river.len(), 5);
    }
}
