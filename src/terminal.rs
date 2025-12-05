// Function to get input from x player
// This will be called from other functions
// This will contain an input validator
// If input invalid, reprompt here
// The main thing I gotta figure out is how to prompt a speicifc player.

use crate::{Action, Game, entities::player::Player};
use text_io::read;
pub fn get_input(given_game: &Game, player_ref: &Player) -> Result<Action, String> {
    // Get valid actions for player and only display those.
    loop {
        println!("You are player number {}", given_game.get_table_pos());
        println!(
            "Your hand is {:?}",
            given_game.all_players[given_game.get_table_pos()].get_hand()
        );
        println!(
            "Your current bet is: {} chips",
            given_game.all_players[given_game.get_table_pos()].current_bet
        );
        println!(
            "Your current chip stack is: {} chips",
            given_game.all_players[given_game.get_table_pos()].get_chips()
        );
        println!("The river contains: {:?}", given_game.get_river());
        println!("_____________________________________________________");
        // println!("Input for player {:?}", player_ref);
        println!("Enter a number corresponding to a choice from below:");
        println!("  0. Fold");
        println!("  1. Check");
        println!("  2. Call");
        println!("  3. Bet");
        println!("  4. Raise");
        println!("  5. All In");
        let player_input: usize = read!();
        let given_action = match player_input {
            0 => Ok(Action::Fold),
            1 => Ok(Action::Check),
            2 => Ok(Action::Call),
            3 => {
                println!("Enter a value to bet:");
                let bet_value: u32 = read!();
                Ok(Action::Bet { value: bet_value })
            }
            4 => {
                println!("Enter a value to raise by:");
                let raise_value: u32 = read!();
                Ok(Action::Raise { value: raise_value })
            }
            5 => Ok(Action::AllIn),
            _ => Err(String::from("IDK")),
        };
        if given_action.is_err()
            || given_game
                .validate_player_action(given_action.clone().unwrap(), player_ref.clone())
                .is_err()
        {
            println!("Err: {:?}", given_action);
            continue;
        } else if given_game
            .validate_player_action(given_action.clone().unwrap(), player_ref.clone())
            .unwrap()
            != given_action.clone().unwrap()
        {
            println!("Err: {:?}", given_action);
            continue;
        } else {
            println!("Ok: {:?}", given_action);
            return given_action;
        }
    }
}
