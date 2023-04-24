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

struct Player {
    name: String,
    score: u32,
    hand: Vec<Card>,
}

struct Game {
    player_a: Player,
    player_b: Player,
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
        }
    }
}

#[cfg(test)]
mod game_tests {
    use crate::card::Card;
    use crate::game::{Game, NotRandomHandGenerator};
    use crate::owner::Owner;
    use crate::strength::Strength;

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
}