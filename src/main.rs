use crate::entities::player::Player;
pub(crate) use crate::game::{Game, betting::Action};

mod entities;
mod game;
mod terminal;
/*
TODO:
 - Add print functions everywhere needed
 - Write out a simple user interface!
 - Add all-in logic
 - ADD TESTS!!!!!
 - Player Identification
 - Once I have all of the above completed AND a working MVP, start Tauri!!!
 Genuinely really looking forward to Tauri.

*/
fn main() {
    let mut game_instance = Game::default();
    println!("{:?}", game_instance.get_current_players());
    loop {
        full_round(&mut game_instance);
        // Ends game when a single player is left.
        game_instance.deck.reset_deck();
        game_instance.clear_bets();
        game_instance.clear_hands();
        game_instance.reset_river();
        game_instance.reset_folded();
        game_instance.reset_all_in();
        if game_instance.get_current_players().len() <= 1 {
            println!("Yeah its over");
            break;
        }
    }
    println!("{:?}", game_instance);
}

fn full_round(game: &mut Game) {
    //Blank hand thingy < I wrote this before. I am looking back now and I have no idea what "blank hand thingy" means.
    //TODO: Create UI essentials here
    // EXAMPLE LAYOUT BELOW
    /*
    This is hand #.
    Player ##
    Your hand is: ##
    Your current bet is: ##
    Your Current Stack is: ##
    The River Contains: _ _ _ _ _
    Player Info:
    Player Num | Current Bet | Current Chips
    -----------------------------------------
     */
    game.create_player_reference_vec();
    game.collect_ante_blind();
    game.deal_cards();
    println!("Preflop:");
    loop {
        let result = sub_round(game);
        if result.is_err() {
            break;
        }
    }
    game.create_player_reference_vec();
    game.progress_river();
    println!("Flop:");
    loop {
        let result = sub_round(game);
        if result.is_err() {
            break;
        }
    }
    game.create_player_reference_vec();
    game.progress_river();
    println!("Turn");
    loop {
        let result = sub_round(game);
        if result.is_err() {
            break;
        }
    }
    println!("River");
    game.create_player_reference_vec();
    game.progress_river();
    loop {
        let result = sub_round(game);
        if result.is_err() {
            break;
        }
    }
    game.create_player_reference_vec();
    game.distribute_winnings();
}

fn sub_round(game: &mut Game) -> Result<u32, String> {
    let result = game.get_next_player(); //Checks to see if round ended.
    if result.is_err() {
        println!("All bets are equal! Continue!");
        return result;
    }
    let player_index = result.unwrap();
    let current_action = get_player_action(&game, player_index); //Gets a valid action from the current player
    let res = game
        .execute_player_action(player_index as usize, current_action)
        .expect("Input should be validated, so no err should occur.");
    return game.go_next_player();
}

fn get_player_action(given_game: &Game, player_index: u32) -> Action {
    let given_player = &given_game.get_player(player_index as usize);
    loop {
        let given_action = terminal::get_input(&given_game, given_player);
        if given_action.is_ok() {
            return given_action.unwrap();
        } else {
            continue;
        }
    }
}
