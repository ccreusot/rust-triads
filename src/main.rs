mod board;
mod card;
mod command;
mod game;
mod player;
mod rules;
mod state;

// execute(rules, game, command)
// Test de push : Pour game1 et command1 => function1 est appelée avec arg1, arg2, etc.
// Test function1 : Appel de function1 avec arg1, arg2, arg3, etc. => res1
use crate::board::Board;
use crate::command::Command;
use crate::game::Game;
use crate::rules::{Rules, RulesImpl};
use crate::state::State;

pub fn execute(rules: &impl Rules, game: Game, command: Command) -> Game {
    match game.state {
        State::WaitingForPlayers { .. } => {
            execute_command_for_waiting_for_player_state(rules, game, command)
        }
        State::WaitingForCards { .. } => {
            execute_command_for_waiting_for_card_state(rules, game, command)
        }
        State::WaitingForPlayerToPlay { .. } => {
            execute_command_for_waiting_for_player_to_play(rules, game, command)
        }
        _ => panic!("State {:?}: Not implemented yet", game.state),
    }
}

fn execute_command_for_waiting_for_player_state(
    rules: &impl Rules,
    game: Game,
    command: Command,
) -> Game {
    match command {
        Command::Register { name } => rules.register_player(game, name),
        _ => game.clone(),
    }
}

fn execute_command_for_waiting_for_card_state(
    rules: &impl Rules,
    game: Game,
    command: Command,
) -> Game {
    match command {
        Command::SelectCard { card_id } => rules.select_card(game, card_id),
        _ => game.clone(),
    }
}

fn execute_command_for_waiting_for_player_to_play(
    rules: &impl Rules,
    game: Game,
    command: Command,
) -> Game {
    match command {
        Command::Play { card_id, x, y } => rules.play_card(game, card_id, x, y),
        _ => game.clone(),
    }
}

fn main() {
    use std::io::{self, Write};

    let mut game = Game::new();
    let rules_set = RulesImpl {};
    loop {
        match game.state {
            State::WaitingForPlayers { count } => {
                print!("Who is player #{}? ", 3 - count);
                io::stdout().flush().unwrap();
                let mut buffer = String::new();
                let stdin = io::stdin();
                match stdin.read_line(&mut buffer) {
                    Ok(_) => {
                        game = execute(
                            &rules_set,
                            game,
                            Command::Register {
                                name: buffer.trim_end().to_string(),
                            },
                        );
                    }
                    _ => return,
                }
            }
            State::WaitingForCards {
                player_count,
                ref deck,
            } => {
                let current_player = game.players[usize::from(2 - player_count)].clone();
                println!("{}'s turn", current_player.name);
                println!("Your hand:");
                for card in current_player.hand.iter() {
                    println!("{}", card);
                }
                println!("Pick a card from the deck:");
                for (index, card) in deck.iter().enumerate() {
                    println!("Card {}:\n{}", index + 1, card);
                }

                loop {
                    print!("Your choice: ");
                    io::stdout().flush().unwrap();
                    let mut buffer = String::new();
                    let stdin = io::stdin();
                    match stdin.read_line(&mut buffer) {
                        Ok(_) => {
                            let cleaned_buffer = buffer.trim().to_string();
                            if let Ok(card_index) = cleaned_buffer.parse::<usize>() {
                                let card = deck[card_index - 1].clone();
                                println!("You choose card #{}", card_index);
                                game = execute(
                                    &rules_set,
                                    game,
                                    Command::SelectCard { card_id: card.id },
                                );
                                break;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            // State::WaitingForPlayerToPlay
            _ => panic!("Game is in an unexpected state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::board::Board;
    use crate::card::Card;
    use crate::command::Command;
    use crate::game::Game;
    use crate::player::Player;
    use crate::rules::RulesImpl;
    use crate::state::State;
    use crate::{execute, player};

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

        game = execute(
            &rules,
            game,
            Command::Register {
                name: "Player 1".to_string(),
            },
        );

        assert_eq!(game.state, State::WaitingForPlayers { count: 1 });
        assert_eq!(
            game.players,
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![]
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
        game = execute(
            &rules,
            game,
            Command::Register {
                name: "Player 1".to_string(),
            },
        );
        game = execute(
            &rules,
            game,
            Command::Register {
                name: "Player 2".to_string(),
            },
        );

        // Then
        match game.state {
            State::WaitingForCards { player_count, deck } => {
                assert_eq!(player_count, 2);
                assert_eq!(deck.len(), 10);
            }
            _ => assert!(false),
        }

        assert_eq!(
            game.players,
            vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![]
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![]
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

        game = execute(
            &rules,
            game,
            Command::Register {
                name: "Player 1".to_string(),
            },
        );

        // When
        let game2 = execute(
            &rules,
            game.clone(),
            Command::Register {
                name: "Player 1".to_string(),
            },
        );

        // Then
        assert_eq!(game.state, game2.state);

        assert_eq!(
            game2.players,
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![]
            }]
        );
    }

    #[test]
    fn when_the_game_is_in_waiting_for_player_it_should_only_process_register_command() {
        // Given
        let rules = RulesImpl {};
        let mut game = Game::new();

        game = execute(
            &rules,
            game,
            Command::Register {
                name: "Player 1".to_string(),
            },
        );

        // When
        game = execute(&rules, game, Command::ChoosePlayer);

        // Then
        assert_eq!(game.state, State::WaitingForPlayers { count: 1 });

        assert_eq!(
            game.players,
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![]
            }]
        );
    }

    #[test]
    fn when_the_game_is_waiting_for_players_to_pick_there_card_player_one_select_a_card() {
        // Given
        let rules = RulesImpl {};
        let mut game = Game {
            board: Board::new(),
            players: vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![],
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![],
                },
            ],
            state: State::WaitingForCards {
                player_count: 2,
                deck: vec![
                    Card {
                        id: "0".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4,
                    },
                    Card {
                        id: "1".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4,
                    },
                    Card {
                        id: "2".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4,
                    },
                    Card {
                        id: "3".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4,
                    },
                ],
            },
        };

        // When
        game = execute(
            &rules,
            game,
            Command::SelectCard {
                card_id: "0".to_string(),
            },
        );

        let game2 = execute(
            &rules,
            game.clone(),
            Command::SelectCard {
                card_id: "3".to_string(),
            },
        );

        // Then
        assert_eq!(
            game.players[0].hand,
            vec![Card {
                id: "0".to_string(),
                top: 1,
                bottom: 2,
                left: 3,
                right: 4
            },]
        );
        assert_eq!(
            game.state,
            State::WaitingForCards {
                player_count: 2,
                deck: vec![
                    Card {
                        id: "1".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4
                    },
                    Card {
                        id: "2".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4
                    },
                    Card {
                        id: "3".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4
                    },
                ]
            }
        );
        assert_eq!(
            game2.players[0].hand,
            vec![
                Card {
                    id: "0".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
                Card {
                    id: "3".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
            ]
        );
        assert_eq!(
            game2.state,
            State::WaitingForCards {
                player_count: 2,
                deck: vec![
                    Card {
                        id: "1".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4
                    },
                    Card {
                        id: "2".to_string(),
                        top: 1,
                        bottom: 2,
                        left: 3,
                        right: 4
                    },
                ]
            }
        );
    }

    #[test]
    fn when_the_player_have_four_card_in_his_hand_it_pick_his_last_card_the_game_should_wait_for_the_next_player(
    ) {
        // Given
        let rules = RulesImpl {};
        let mut game = Game {
            board: Board::new(),
            players: vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![
                        Card {
                            id: "0".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "1".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "2".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "3".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                    ],
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![],
                },
            ],
            state: State::WaitingForCards {
                player_count: 2,
                deck: vec![Card {
                    id: "4".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4,
                }],
            },
        };

        // When
        game = execute(
            &rules,
            game,
            Command::SelectCard {
                card_id: "4".to_string(),
            },
        );

        // Then
        assert_eq!(
            game.players[0].hand,
            vec![
                Card {
                    id: "0".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
                Card {
                    id: "1".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
                Card {
                    id: "2".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
                Card {
                    id: "3".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
                Card {
                    id: "4".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4
                },
            ]
        );
        if let State::WaitingForCards { player_count, deck } = game.state {
            assert_eq!(player_count, 1);
            assert_eq!(deck.len(), 10);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn when_all_players_have_their_cards_selected_should_wait_for_the_player_selected_to_play_first(
    ) {
        let rules = RulesImpl {};
        let mut game = Game {
            board: Board::new(),
            players: vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![
                        Card {
                            id: "0".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "1".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "2".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "3".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "4".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                    ],
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![
                        Card {
                            id: "0".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "1".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "2".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "3".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                    ],
                },
            ],
            state: State::WaitingForCards {
                player_count: 1,
                deck: vec![Card {
                    id: "4".to_string(),
                    top: 1,
                    bottom: 2,
                    left: 3,
                    right: 4,
                }],
            },
        };

        // When
        game = execute(
            &rules,
            game,
            Command::SelectCard {
                card_id: "4".to_string(),
            },
        );

        if let State::WaitingForPlayerToPlay { player_name } = game.state {
            assert_eq!(player_name, game.players[0].name);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn when_the_first_player_play_a_card_it_should_remove_the_card_from_his_hand_and_update_the_board(
    ) {
        let rules = RulesImpl {};
        let mut game = Game {
            board: Board::new(),
            players: vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![
                        Card {
                            id: "0".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "1".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "2".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "3".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "4".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                    ],
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![
                        Card {
                            id: "0".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "1".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "2".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "3".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                        Card {
                            id: "4".to_string(),
                            top: 1,
                            bottom: 2,
                            left: 3,
                            right: 4,
                        },
                    ],
                },
            ],
            state: State::WaitingForPlayerToPlay {
                player_name: "Player 1".to_string(),
            },
        };

        let card_id = game.players[0].hand[0].id.clone();
        game = execute(
            &rules,
            game,
            Command::Play {
                card_id: card_id.clone(),
                x: 0,
                y: 0,
            },
        );

        if let State::WaitingForPlayerToPlay { player_name } = game.state {
            assert_eq!(player_name, game.players[1].name);
            assert_eq!(game.players[0].hand.len(), 4);
            let board = game.board.clone();

            assert_eq!(board.get_cell_owner(0, 0).unwrap(), "Player 1");
            assert_eq!(board.get_card_at(0, 0).unwrap().unwrap().id, card_id);
        } else {
            assert!(false);
        }
    }

    fn test_capture_param(played_card_pos: (u8, u8), check_capture_pos: (u8, u8)) {
        let player_card_index: u8 = played_card_pos.0 * 3 + played_card_pos.1;
        let mut cards = vec![None, None, None, None, None, None, None, None, None];
        cards[usize::from(player_card_index)] = Some(Card {
            id: "4".to_string(),
            top: 1,
            bottom: 2,
            left: 3,
            right: 4,
        });

        let rules = RulesImpl {};
        let mut game = Game {
            board: Board {
                cards: cards,
                cell_owner: HashMap::from([(player_card_index, "Player 1".to_string())]),
            },
            players: vec![
                Player {
                    name: "Player 1".to_string(),
                    hand: vec![],
                },
                Player {
                    name: "Player 2".to_string(),
                    hand: vec![Card {
                        id: "0".to_string(),
                        top: 5,
                        bottom: 5,
                        left: 5,
                        right: 5,
                    }],
                },
            ],
            state: State::WaitingForPlayerToPlay {
                player_name: "Player 2".to_string(),
            },
        };

        let card_id = game.players[1].hand[0].id.clone();
        game = execute(
            &rules,
            game,
            Command::Play {
                card_id: card_id.clone(),
                x: check_capture_pos.0,
                y: check_capture_pos.1,
            },
        );

        let board = game.board.clone();

        assert_eq!(
            board
                .get_cell_owner(played_card_pos.0, played_card_pos.1)
                .unwrap(),
            "Player 2"
        );
    }

    #[test]
    fn when_a_card_is_played_with_a_higher_value_that_one_of_his_neighbour_it_should_capture_it() {
        test_capture_param((0, 0), (1, 0)); // test capture from left side
        test_capture_param((0, 0), (0, 1)); // test capture from top side
        test_capture_param((1, 0), (0, 0)); // test capture from right side
        test_capture_param((0, 1), (0, 0)); // test capture from bottom side
    }
}
