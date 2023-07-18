use crate::game::Game;
use crate::state::State;
use crate::player::Player;
use crate::card::Card;

pub trait Rules {
    // Setup rules
    fn register_player(game: Game, name: String) -> Game;

    // Game rules
}

struct RulesImpl;

impl Rules for RulesImpl {
    fn register_player(game: Game, name: String) -> Game {
        if let State::WaitingForPlayers { count } = game.state {
            if count - 1 == 0 {
                if game.players[0].name == name {
                    return game.clone();
                }

                let mut _players = game.players.clone();
                _players.push(Player {
                    name,
                    hand: vec![],
                    owned_played_card: vec![],
                });

                // TODO: Generate cards for the first players
                return Game {
                    state: State::WaitingForCards {
                        playerCount: 2,
                        deck: generate_deck_of(10),
                     },
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
        return game.clone();
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

fn generate_deck_of(count: u8) -> Vec<Card> {
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