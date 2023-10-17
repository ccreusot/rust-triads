use std::collections::HashMap;

use crate::card::Card;
use crate::player::Player;

#[derive(Clone, Debug)]
pub struct Board {
    pub cards: Vec<Option<Card>>,
    pub cell_owner: HashMap<u8, String>
}

impl Board {
    pub fn new() -> Board {
        Board {
            cards: vec![None; 9], // Use 9 because of 3x3 size
            cell_owner: HashMap::new()
        }
    }

    pub fn get_card_at(&self, x: u8, y: u8) -> Result<Option<Card>, String> {
        let index: usize = usize::from((x * 3) + y);
        if index >= 9 {
            return Err("Out of bounds".to_string());
        }
        return Ok(self.cards[index].clone());
    }

    pub fn set_card_at(&self, player: &Player, card: &Card, x: u8, y: u8) -> Board {
        let index = usize::from((x * 3) + y);
        if self.get_card_at(x, y) != Ok(None) {
            return self.clone();
        }

        let mut new_cards = self.cards.clone();
        new_cards[index] = Option::Some(card.clone());

        let mut new_board = self.clone();
        new_board.cards = new_cards;

        let mut new_cell_owner = self.cell_owner.clone();
        new_cell_owner.insert(index as u8, player.name.clone());
        new_board.cell_owner = new_cell_owner;

        return new_board;
    }

    pub fn get_cell_owner(&self, x: u8, y: u8) -> Result<String, String> {
        let index = (x * 3) + y;
        if index >= 9 {
            return Err("Out of bounds".to_string());
        }
        return Ok(self.cell_owner.get(&index).unwrap().clone());
    }
}
