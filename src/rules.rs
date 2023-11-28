use crate::card::Card;
use crate::game::Game;
use crate::player::Player;
use crate::state::State;

pub trait Rules {
    // Setup rules

    // In state: WaitingForPlayer
    // Switch state: WaitingForPlayers.count == 0
    // To state: WaitingForCards (players_count: 2, deck: Card[])
    fn register_player(&self, game: Game, name: String) -> Game;

    // In state: WaitingForCards
    // Switch state: WaitingForCards.players_count == 0 (5 cards per player)
    // To state: WaitingForCards || WaitingForPlayerToPlay
    fn select_card(&self, game: Game, card_id: String) -> Game;

    // Game rules

    // In state: WaitingForPlayerToPlay
    // Switch state: WaitingForPlayerToPlay (next player)
    // To state: WaitingForPlayerToPlay
    fn play_card(&self, game: Game, card_id: String, x: u8, y: u8) -> Game;

    fn check_neighbour_cards_to_current_position(
        &self,
        game: &Game,
        card: &Card,
        card_position: (u8, u8),
        check_offset: (i8, i8),
    ) -> bool;
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
        // print!("{:?}", value);
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
                _players.push(Player { name, hand: vec![] });

                // TODO: Generate cards for the first players
                return Game {
                    state: State::WaitingForCards {
                        player_count: 2,
                        deck: generate_deck_of(10),
                    },
                    players: _players,
                    board: game.board.clone(),
                };
            }
            return Game {
                state: State::WaitingForPlayers { count: count - 1 },
                players: vec![Player { name, hand: vec![] }],
                board: game.board.clone(),
            };
        }
        return game.clone();
    }

    fn select_card(&self, game: Game, card_id: String) -> Game {
        if let State::WaitingForCards { player_count, deck } = game.state {
            let player_index = (2 - player_count) as usize;
            let mut _players = game.players.clone();
            let mut _deck = deck.clone();

            if let Some(index) = _deck.iter().position(|card| card.id == card_id) {
                let card = _deck.remove(index);
                _players[player_index].hand.push(card);
            }

            if player_count == 1 && _players[player_index].hand.len() == 5 {
                return Game {
                    state: State::WaitingForPlayerToPlay {
                        player_name: _players[0].name.to_string(),
                    },
                    players: _players,
                    board: game.board.clone(),
                };
            }

            if _players[player_index].hand.len() == 5 {
                return Game {
                    state: State::WaitingForCards {
                        player_count: player_count - 1,
                        deck: generate_deck_of(10),
                    },
                    players: _players,
                    board: game.board.clone(),
                };
            }

            return Game {
                state: State::WaitingForCards {
                    player_count,
                    deck: _deck,
                },
                players: _players,
                board: game.board.clone(),
            };
        }
        return game.clone();
    }

    fn play_card(&self, game: Game, card_id: String, x: u8, y: u8) -> Game {
        if let State::WaitingForPlayerToPlay { player_name } = &game.state {
            let player = game
                .players
                .iter()
                .find(|player| player.name == *player_name);
            let current_player_id = game
                .players
                .iter()
                .position(|player| player.name == *player_name);
            let next_player_name = game.players
                [(current_player_id.unwrap() + 1) % game.players.len()]
            .name
            .clone();

            if let None = player {
                return game.clone();
            }

            let player = player.unwrap();

            let card = player.hand.iter().find(|card| card.id == card_id);

            if let None = card {
                return game.clone();
            }

            let card = card.unwrap();

            let updated_player = player.drop_card(card);
            let mut updated_board = game.board.set_card_at(&updated_player, card, x, y);

            let offsets: Vec<(i8, i8)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

            offsets.iter().for_each(|offset| {
                if self.check_neighbour_cards_to_current_position(&game, card, (x, y), *offset) {
                    let (x_offset, y_offset) = *offset;

                    updated_board = updated_board.set_cell_owner(
                        updated_player.clone(),
                        (x as i8 + x_offset) as u8,
                        (y as i8 + y_offset) as u8,
                    );
                }
            });

            let mut new_state = State::WaitingForPlayerToPlay {
                player_name: next_player_name,
            };
            if !updated_board.cards.iter().any(|card| *card == None) {
                let scores: (u8, u8) = (
                    updated_board.cell_owner.iter().filter(|(_, player)| player.name == game.players[0].name.clone()).count() as u8,
                    updated_board.cell_owner.iter().filter(|(_, player)| player.name == game.players[1].name.clone()).count() as u8
                );
                new_state = State::EndOfGame {
                    scores: scores,
                    winner: if scores.0 > scores.1 { game.players[0].name.clone() } else { game.players[1].name.clone() }
                }
            }

            let updated_game = Game {
                players: game
                    .players
                    .iter()
                    .map(|player| {
                        if player.name == *player_name {
                            return updated_player.clone();
                        }
                        return player.clone();
                    })
                    .collect::<Vec<Player>>(),
                state: new_state.clone(),
                board: updated_board.clone(),
            };
            return updated_game;
        }
        return game.clone();
    }

    fn check_neighbour_cards_to_current_position(
        &self,
        game: &Game,
        card: &Card,
        card_position: (u8, u8),
        check_offset: (i8, i8),
    ) -> bool {
        let (x_card, y_card) = card_position;
        let (x_check_offset, y_check_offset) = check_offset;
        let x_check = x_card as i8 + x_check_offset;
        let y_check = y_card as i8 + y_check_offset;

        // TODO: Test and handle unauthorized diagonal check

        if x_check < 0 || x_check >= 3 || y_check < 0 || y_check >= 3 {
            return false;
        }

        let opposed_card = game.board.get_card_at(x_check as u8, y_check as u8);

        if let Ok(Some(opposed_card)) = opposed_card {
            match check_offset {
                (-1, 0) => return card.left > opposed_card.right,
                (1, 0) => return card.right > opposed_card.left,
                (0, -1) => return card.top > opposed_card.bottom,
                (0, 1) => return card.bottom > opposed_card.top,
                _ => return false,
            }
        }

        return false;
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
