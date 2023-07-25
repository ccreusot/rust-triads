use crate::game::Game;
use crate::state::State;
use crate::player::Player;
use crate::card::Card;

pub trait Rules {
    // Setup rules

    // In state: WaitingForPlayer
    // Switch state: WaitingForPlayers.count == 0
    // To state: WaitingForCards (players_count: 2, deck: Card[])
    fn register_player(&self, game: Game, name: String) -> Game;

    // In state: WaitingForCards
    // Switch state: WaitingForCards.players_count == 0 (5 cards per player)
    // To state: ???
    fn select_card(&self, game: Game, card_id: String) -> Game;

    // Game rules
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

pub struct RulesImpl;

impl Rules for RulesImpl {
    fn register_player(&self, game: Game, name: String) -> Game {
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

    fn select_card(&self, game: Game, card_id: String) -> Game {
        if let State::WaitingForCards { playerCount, deck } = game.state {
            let player_index = (2 - playerCount) as usize;
            let mut _players = game.players.clone();
            let mut _deck = deck.clone();

            if let Some(index) = _deck.iter().position(|card| card.id == card_id) {
                let card = _deck.remove(index);
                _players[player_index].hand.push(card);    
            }

            if _players[player_index].hand.len() == 5 {
                return Game {
                    state: State::WaitingForCards {
                        playerCount: playerCount - 1,
                        deck: generate_deck_of(10),
                    },
                    players: _players,
                };
            }
            
            return Game {
                state: State::WaitingForCards {
                    playerCount,
                    deck: _deck,
                },
                players: _players,
            };
        }
        return game.clone(); 
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card::Card;

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