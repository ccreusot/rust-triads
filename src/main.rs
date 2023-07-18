mod game;
mod player;
mod rules;
mod state;
mod board;
mod command;

// execute(rules, game, command)
// Test de push : Pour game1 et command1 => function1 est appelée avec arg1, arg2, etc.
// Test function1 : Appel de function1 avec arg1, arg2, arg3, etc. => res1
use crate::game::Game;
use crate::command::Command;
use crate::rules::Rules;
use crate::state::State;


pub fn execute(rules: dyn Rules, game: Game, command: Command) -> Game {
    match game.state {
        State::WaitingForPlayers { .. } => {
            handle_command_for_waiting_for_player_state(command)
        }
        _ => panic!("State {:?}: Not implemented yet", game.state),
    }
}

fn handle_command_for_waiting_for_player_state(rules: dyn Rules, game: Game, command: Command) -> Game {
    match command {
        Command::Register { name } => rules.register_player(name),
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
    use crate::game::Game;
    use crate::state::State;
    use crate::command::Command;
    use crate::player::Player;
    use crate::card::Card;
//    use crate::rules::Rules;

    #[test]
    fn on_creation_game_should_be_in_waiting_for_players_state() {
        let game = Game::new();

        assert_eq!(game.state, State::WaitingForPlayers { count: 2 });
    }

    #[test]
    fn when_we_push_one_registering_command_to_the_game_we_should_get_a_state_waiting_for_player_one(
    ) {
        let mut game = Game::new();

        game = game.push(Command::Register {
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
        let mut game = Game::new();

        // When
        game = game.push(Command::Register {
            name: "Player 1".to_string(),
        });
        game = game.push(Command::Register {
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
        let mut game = Game::new();

        game = game.push(Command::Register {
            name: "Player 1".to_string(),
        });

        // When
        let game2 = game.push(Command::Register {
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
        let mut game = Game::new();

        game = game.push(Command::Register {
            name: "Player 1".to_string(),
        });

        // When
        game = game.push(Command::ChoosePlayer);

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

    #[test]
    fn test_card_has_valid_values() {
        let card: Card = generate_card(15).unwrap();

        print!("{:?}", card);
        assert!(card.top >= 1 && card.top <= 10);
        assert!(card.right >= 1 && card.right <= 10);
        assert!(card.bottom >= 1 && card.bottom <= 10);
        assert!(card.left >= 1 && card.left <= 10);
    }

    #[test]
    fn test_card_can_not_have_value_under_15() {
        for i in 1..14 {
            let card = generate_card(i);

            if let Err(_) = card {
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_card_can_not_have_value_above_25() {
        for i in 26..100 {
            let card = generate_card(i);

            if let Err(_) = card {
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_card_has_valid_sum() {
        let card = generate_card(20).unwrap();

        assert_eq!(card.sum(), 20);
    }

    #[test]
    fn test_generate_card_does_not_generate_the_same_card_twice() {
        assert_ne!(generate_card(15).unwrap().id, generate_card(15).unwrap().id);
    }

    #[test]
    fn test_generate_deck_of_10_cards() {
        let deck = generate_deck_of(10);

        assert_eq!(deck.len(), 10);
    }
}
