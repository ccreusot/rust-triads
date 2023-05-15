use crate::board::Board;
use crate::card::Card;
use crate::owner::Owner;
use crate::strength::Strength;

trait HandGenerator {
    fn generate_hand(&self, owner: Owner) -> Vec<Card>;
}

struct RandomHandGenerator;

impl HandGenerator for RandomHandGenerator {
    fn generate_hand(&self, _: Owner) -> Vec<Card> {
        vec![]
    }
}

struct NotRandomHandGenerator {
    hand: Vec<Card>,
}

impl NotRandomHandGenerator {
    fn new(hand: Vec<Card>) -> NotRandomHandGenerator {
        NotRandomHandGenerator {
            hand,
        }
    }
}

impl HandGenerator for NotRandomHandGenerator {
    fn generate_hand(&self, owner: Owner) -> Vec<Card> {
        self.hand.clone().into_iter().map(|card| Card { owner, ..card }).collect()
    }
}

#[derive(Clone)]
struct Player {
    name: String,
    score: u32,
    hand: Vec<Card>,
}

impl Player {
    fn remove_card(&self, card: Card) -> Player {
        let mut player = self.clone();
        player.hand = player.hand.into_iter().filter(|c| *c != card).collect();
        player
    }
}

#[derive(Clone)]
struct Game {
    player_a: Player,
    player_b: Player,
    board: Board,
}

impl Game{
    fn new(hand_generator: impl HandGenerator) -> Game {
        Game {
            player_a: Player {
                name: "Player A".to_string(),
                score: 0,
                hand: hand_generator.generate_hand(Owner::PlayerOne),
            },
            player_b: Player {
                name: "Player B".to_string(),
                score: 0,
                hand: hand_generator.generate_hand(Owner::PlayerTwo),
            },
            board: Board::new(),
        }
    }

    fn player_input(&self, row: usize, column: usize, card: Card) -> Game {
        let board = self.board.place_card(row, column, card);
        if card.owner == Owner::PlayerOne {
            let mut game = self.clone();
            game.player_a =  self.player_a.remove_card(card);
            game.board = board;
            game
        } else {
            let mut game = self.clone();
            game.player_b =  self.player_b.remove_card(card);
            game.board = board;
            game
        }
    }
}

#[cfg(test)]
mod game_tests {
    use crate::card::Card;
    use crate::game::{Game, NotRandomHandGenerator};
    use crate::owner::Owner;
    use crate::strength::Strength;
    use crate::cell::Cell;

    #[test]
    fn test_has_2_players() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let game = Game::new(hand_generator);
        assert_eq!(game.player_a.name, "Player A");
        assert_eq!(game.player_a.hand, vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        assert_eq!(game.player_b.name, "Player B");
        assert_eq!(game.player_b.hand, vec![
            Card { owner: Owner::PlayerTwo, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
    }

    #[test]
    fn test_player_one_is_the_current_player_at_start() {
        let hand_generator = NotRandomHandGenerator::new(vec![
            Card { owner: Owner::PlayerOne, top: Strength::Two, left: Strength::Two, bottom: Strength::Two, right: Strength::Two },
        ]);
        let mut game = Game::new(hand_generator);

        game = game.player_input(1, 1, game.player_a.hand[0]);

        if let Cell::Card { card } = game.board.get_cell(1, 1) {
            assert_eq!(card.owner, Owner::PlayerOne);
        }

        assert_eq!(game.player_a.hand, vec![]);
    }
}