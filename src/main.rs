
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
        match command {
            Command::Register { name } => self.register_player(name),
            _ => panic!("Command {:?}: Not implemented yet", command),
        }
    }

    fn register_player(&self, name: String) -> Game {
        if let State::WaitingForPlayers { count } = self.state {
            if count - 1 == 0 {
                if self.players[0].name == name {
                    return self.clone();
                }

                let mut _players = self.players.clone();
                _players.push(Player{name, hand: vec![], owned_played_card: vec![]});

                return Game {
                    state: State::WaitingForCards { playerCount: 2 },
                    players: _players
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

#[derive(Clone, Debug, PartialEq)]
struct Player {
    // id: u8,
    // score: u32,
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
            vec![Player {
                name: "Player 1".to_string(),
                hand: vec![],
                owned_played_card: vec![]
            }, Player {
                name: "Player 2".to_string(),
                hand: vec![],
                owned_played_card: vec![]}]
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
        // TODO : Improve test after another command is implemented
        let mut game = Game::new();

        game = game.push(Command::Register {
            name: "Player 1".to_string(),
        });

        game = game.push(Command::ChoosePlayer);
        
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
