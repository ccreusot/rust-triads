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
        },
        State::WaitingForCards { .. } => {
            execute_command_for_waiting_for_card_state(rules, game, command)
        },
        _ => panic!("State {:?}: Not implemented yet", game.state),
    }
}

fn execute_command_for_waiting_for_player_state(rules: &impl Rules, game: Game, command: Command) -> Game {
    match command {
        Command::Register { name } => rules.register_player(game, name),
        _ => game.clone(),
    }
}

fn execute_command_for_waiting_for_card_state(rules: &impl Rules, game: Game, command: Command) -> Game {
    match command {
        Command::SelectCard { card_id } => rules.select_card(game, card_id),
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
    use crate::{execute, player};
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

    #[test]
    fn when_the_game_is_waiting_for_players_to_pick_there_card_player_one_select_a_card() {
        // Given
        let rules = RulesImpl {};
        let mut game = Game {
            players: vec![
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
            ],
            state: State::WaitingForCards { playerCount: 2, deck: vec![
                Card {id:"0".to_string(), top: 1, bottom: 2, left: 3, right: 4},
                Card {id:"1".to_string(), top: 1, bottom: 2, left: 3, right: 4},
                Card {id:"2".to_string(), top: 1, bottom: 2, left: 3, right: 4},
                Card {id:"3".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
            ]},
        };

        // When
        game = execute(&rules, game, Command::SelectCard { card_id:"0".to_string() });

        let game2 = execute(&rules, game.clone(), Command::SelectCard { card_id:"3".to_string() });

        // Then
        assert_eq!(game.players[0].hand, vec![
            Card {id:"0".to_string(), top: 1, bottom: 2, left: 3, right: 4},
        ]);
        assert_eq!(game.state, State::WaitingForCards { playerCount: 2, deck: vec![
            Card {id:"1".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"2".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"3".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
        ]});
        assert_eq!(game2.players[0].hand, vec![
            Card {id:"0".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"3".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
        ]);
        assert_eq!(game2.state, State::WaitingForCards { playerCount: 2, deck: vec![
            Card {id:"1".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"2".to_string(), top: 1, bottom: 2, left: 3, right: 4},
        ]});
    }

    #[test]
    fn when_the_player_have_four_card_in_his_hand_it_pick_his_last_card_the_game_should_wait_for_the_next_player() {
         // Given
         let rules = RulesImpl {};
         let mut game = Game {
             players: vec![
                 Player {
                     name: "Player 1".to_string(),
                     hand: vec![
                        Card {id:"0".to_string(), top: 1, bottom: 2, left: 3, right: 4},
                        Card {id:"1".to_string(), top: 1, bottom: 2, left: 3, right: 4},
                        Card {id:"2".to_string(), top: 1, bottom: 2, left: 3, right: 4},
                        Card {id:"3".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
                     ],
                     owned_played_card: vec![]
                 },
                 Player {
                     name: "Player 2".to_string(),
                     hand: vec![],
                     owned_played_card: vec![]
                 }
             ],
             state: State::WaitingForCards { playerCount: 2, deck: vec![
                 Card {id:"4".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
             ]},
         };

        // When
        game = execute(&rules, game, Command::SelectCard { card_id:"4".to_string() });

        // Then
        assert_eq!(game.players[0].hand, vec![
            Card {id:"0".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"1".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"2".to_string(), top: 1, bottom: 2, left: 3, right: 4},
            Card {id:"3".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
            Card {id:"4".to_string(), top: 1, bottom: 2, left: 3, right: 4},    
        ]);
        if let State::WaitingForCards { playerCount, deck } = game.state {
            assert_eq!(playerCount, 1);
            assert_eq!(deck.len(), 10);
        } else {
            assert!(false);
        }
    }



}
