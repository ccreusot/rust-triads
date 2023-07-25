mod game;
mod player;
mod rules;
mod state;
mod board;
mod command;
mod card;

// execute(rules, game, command)
// Test de push : Pour game1 et command1 => function1 est appelée avec arg1, arg2, etc.
// Test function1 : Appel de function1 avec arg1, arg2, arg3, etc. => res1
use crate::game::Game;
use crate::command::Command;
use crate::rules::Rules;
use crate::state::State;

pub fn execute(rules: &impl Rules, game: Game, command: Command) -> Game {
    match game.state {
        State::WaitingForPlayers { .. } => {
            execute_command_for_waiting_for_player_state(rules, game, command)
        }
        _ => panic!("State {:?}: Not implemented yet", game.state),
    }
}

fn execute_command_for_waiting_for_player_state(rules: &impl Rules, game: Game, command: Command) -> Game {
    match command {
        Command::Register { name } => rules.register_player(game, name),
        _ => game.clone(),
    }
}


fn main() {
    println!("Hello, world!");
    // Initialize la window
    // Initialize le game
    // While (true)
}

#[cfg(test)]
mod tests {
    use crate::execute;
    use crate::game::Game;
    use crate::state::State;
    use crate::command::Command;
    use crate::player::Player;
    use crate::card::Card;
   use crate::rules::RulesImpl;

    #[test]
    fn on_creation_game_should_be_in_waiting_for_players_state() {
        let game = Game::new();

        assert_eq!(game.state, State::WaitingForPlayers { count: 2 });
    }

    #[test]
    fn when_we_push_one_registering_command_to_the_game_we_should_get_a_state_waiting_for_player_one(
    ) {
        let rules = RulesImpl {};
        let mut game = Game::new();

        game = execute(&rules, game, Command::Register {
            name: "Player 1".to_string(),
        });

        assert_eq!(game.state, State::WaitingForPlayers { count: 1 });
        assert_eq!(
            game.players,
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![],
                owned_played_card: vec![]
            }]
        );
    }

    #[test]
    fn when_we_push_two_registering_commands_to_the_game_we_should_get_a_state_waiting_for_card_for_player_one(
    ) {
        // Given
        let rules = RulesImpl {};
        let mut game = Game::new();

        // When
        game = execute(&rules, game, Command::Register {
            name: "Player 1".to_string(),
        });
        game = execute(&rules, game, Command::Register {
            name: "Player 2".to_string(),
        });

        // Then
        match game.state {
            State::WaitingForCards { playerCount, deck } => 
            {
                assert_eq!(playerCount, 2);
                assert_eq!(deck.len(), 10);
            },
            _ => assert!(false),
        }

        assert_eq!(
            game.players,
            vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![],
                    owned_played_card: vec![]
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![],
                    owned_played_card: vec![]
                }
            ]
        );
    }

    #[test]
    fn when_we_register_a_player_we_should_reject_him_if_his_name_conflicts_with_already_registered_player(
    ) {
        // Given
        let rules = RulesImpl {};
        let mut game = Game::new();

        game = execute(&rules, game, Command::Register {
            name: "Player 1".to_string(),
        });

        // When
        let game2 = execute(&rules, game.clone(), Command::Register {
            name: "Player 1".to_string(),
        });

        // Then
        assert_eq!(game.state, game2.state);

        assert_eq!(
            game2.players,
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![],
                owned_played_card: vec![]
            }]
        );
    }

    #[test]
    fn when_the_game_is_in_waiting_for_player_it_should_only_process_register_command() {
        // Given
        let rules = RulesImpl {};
        let mut game = Game::new();

        game = execute(&rules, game, Command::Register { name: "Player 1".to_string() });

        // When
        game = execute(&rules, game, Command::ChoosePlayer);

        // Then
        assert_eq!(game.state, State::WaitingForPlayers { count: 1 });

        assert_eq!(
            game.players,
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![],
                owned_played_card: vec![]
            }]
        );
    }
}
