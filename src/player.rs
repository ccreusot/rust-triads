use crate::card::Card;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    // pub sign: char,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hand: Vec::new(),
        }
    }
    pub fn drop_card(&self, card: &Card) -> Player {
        let mut new_hand = self.hand.clone();
        new_hand.retain(|_card| _card.id != card.id);
        Player {
            name: self.name.clone(),
            hand: new_hand,
        }
    }
}

// TODO: Make the sign attributed to the player