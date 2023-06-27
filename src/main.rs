#[derive(Clone, Debug)]
enum Command {
    Register { name: String },
    Select { card: Card },
    ChoosePlayer,
    Play { card: Card, x: u8, y: u8 },
    Check { x: u8, y: u8 },
}

#[derive(Clone, PartialEq, Debug)]
enum State {
    WaitingForPlayers { count: u8 },
    WaitingForCards { playerCount: u8 },
}

#[derive(Clone, Debug)]
struct Game {
    state: State,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::WaitingForPlayers { count: 2 },
            players: vec![],
        }
    }

    pub fn push(&self, command: Command) -> Game {
        match self.state {
            State::WaitingForPlayers { .. } => {
                self.handle_command_for_waiting_for_player_state(command)
            }
            _ => panic!("State {:?}: Not implemented yet", self.state),
        }
    }

    fn handle_command_for_waiting_for_player_state(&self, command: Command) -> Game {
        match command {
            Command::Register { name } => self.register_player(name),
            _ => self.clone(),
        }
    }

    fn register_player(&self, name: String) -> Game {
        if let State::WaitingForPlayers { count } = self.state {
            if count - 1 == 0 {
                if self.players[0].name == name {
                    return self.clone();
                }

                let mut _players = self.players.clone();
                _players.push(Player {
                    name,
                    hand: vec![],
                    owned_played_card: vec![],
                });

                return Game {
                    state: State::WaitingForCards { playerCount: 2 },
                    players: _players,
                };
            }
            return Game {
                state: State::WaitingForPlayers { count: count - 1 },
                players: vec![Player {
                    name,
                    hand: vec![],
                    owned_played_card: vec![],
                }],
            };
        }
        return self.clone();
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Card {
    id: String,
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
}

impl Card {
    pub fn sum(&self) -> u8 {
        (self.top + self.right + self.bottom + self.left)
    }
}

fn generate_card(value: u8) -> Result<Card, String> {
    if (value < 15 || value > 25) {
        return Err("Value should be between 15 and 25".to_string());
    }
    
    use rand::Rng;
    use uuid::Uuid;
    use std::cmp::min;

    let mut rng = rand::thread_rng();

    let top = rng.gen_range(1..10);
    let right = rng.gen_range(1..min(10, value - top));
    let bottom = rng.gen_range(1..min(10, value - top - right));
    let left = value - top - right - bottom;

    Ok(Card {
        id: format!("{}", Uuid::new_v4()),
        top: top,
        right: right,
        bottom: bottom,
        left: left,
    })
}

#[derive(Clone, Debug, PartialEq)]
struct Player {
    name: String,
    hand: Vec<Card>,
    owned_played_card: Vec<Card>,
}

struct Board {
    cards: Vec<Vec<Option<Card>>>,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut game = Game::new();

        game = game.push(Command::Register {
            name: "Player 1".to_string(),
        });
        game = game.push(Command::Register {
            name: "Player 2".to_string(),
        });

        assert_eq!(game.state, State::WaitingForCards { playerCount: 2 });

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
        let card = generate_card(15).unwrap();

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
}
