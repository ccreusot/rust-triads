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
    // Todo: When Player count decrement we generate a new list of card for the next player
    WaitingForCards { playerCount: u8, deck: Vec<Card> },
}

#[derive(Clone, Debug)]
struct Game {
    state: State,
    players: Vec<Player>,
    deck_generator: Box<dyn DeckGenerator>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::WaitingForPlayers { count: 2 },
            players: vec![],
            deck_generator: Box::new(DeckGeneratorImpl {}),
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

                // TODO: Generate cards for the first players
                return Game {
                    state: State::WaitingForCards {
                        playerCount: 2,
                        deck: self.deck_generator.generate_deck_of(10),
                     },
                    players: _players,
                    deck_generator: self.deck_generator,
                };
            }
            return Game {
                state: State::WaitingForPlayers { count: count - 1 },
                players: vec![Player {
                    name,
                    hand: vec![],
                    owned_played_card: vec![],
                }],
                deck_generator: self.deck_generator,
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

type Randomizer = fn(u8, u8) -> u8;

fn generate_card_with_randomizer(value: u8, randomizer: Randomizer) -> Result<Card, String> {
    use std::cmp::min;
    use uuid::Uuid;
    
    if value < 15 || value > 25 {
        //print!("Invalid value {:?}, should be between 15 and 25", value);
        return Err("Value should be between 15 and 25".to_string());
    }

    let top = randomizer(1, 10);
    let right = randomizer(1, min(10, value - top - 2));
    let bottom = randomizer(1, min(10, value - top - right - 1));
    let left = value - top - right - bottom;

    return Ok(Card {
        id: format!("{}", Uuid::new_v4()),
        top: top,
        right: right,
        bottom: bottom,
        left: left,
    });
}

fn generate_card(value: u8) -> Result<Card, String> {
    generate_card_with_randomizer(value, |min: u8, max: u8| -> u8 {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    })
}

trait DeckGenerator {
    fn generate_deck_of(self: &Self, count: u8) -> Vec<Card>;
}

use core::fmt::Debug;

impl Debug for dyn DeckGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeckGenerator").finish()
    }
}

use core::clone::Clone;

impl Clone for dyn DeckGenerator {
    fn clone(&self) -> Self {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
struct DeckGeneratorImpl {}

impl DeckGenerator for DeckGeneratorImpl {
    fn generate_deck_of(self: &Self, count: u8) -> Vec<Card> {
        use rand::Rng;
    
        let mut rng = rand::thread_rng();    
    
        let mut deck = vec![];
        for _i in 0..count {
            let value = rng.gen_range(15..26);
            print!("{:?}", value);
            match generate_card(value) {
                Ok(card) => deck.push(card),
                Err(_) => {}
            }
        }
        return deck;
    }
}

#[derive(Clone, Debug)]
struct MockDeckGenerator {}

impl DeckGenerator for MockDeckGenerator {
    fn generate_deck_of(self: &Self, count: u8) -> Vec<Card> {
        let mut deck = vec![];
        for i in 0..count {
            deck.push(Card { id: i.to_string(), top: 10, right: 3, bottom: 1, left: 7 })
        }
        return deck;
    }
}

fn test2(generator: Box<dyn DeckGenerator>) {
    generator.generate_deck_of(1);
}

fn test() {
    let mut mock = Box::new(MockDeckGenerator{});
    let mut concrete = Box::new(DeckGeneratorImpl{});

    test2(mock);
    test2(concrete);
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
        let deck = DeckGeneratorImpl{}.generate_deck_of(10);

        assert_eq!(deck.len(), 10);
    }
}
