use crate::owner::Owner;
use crate::strength::Strength;
use crate::board::Board;
use crate::cell::Cell;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Card {
    pub owner: Owner,
    pub top: Strength,
    pub left: Strength,
    pub bottom: Strength,
    pub right: Strength,
}

impl Card {
    pub fn to_lines(&self) -> [String; 3] {
        [
            format!("   {}   |", self.top.to_char()),
            format!(
                "{}  {}  {}|",
                self.left.to_char(),
                self.owner.to_sign(),
                self.right.to_char()
            ),
            format!("   {}   |", self.bottom.to_char()),
        ]
    }

    pub fn get_bottom_neighbor(&self, board: &Board, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = board.get_cell(row + 1, column) {
           return Some(card);
       }
       return None
    }

    pub fn get_top_neighbor(&self, board: &Board, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = board.get_cell(row - 1, column) {
           return Some(card);
       }
       return None
    }

    pub fn get_left_neighbor(&self, board: &Board, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = board.get_cell(row, column - 1) {
           return Some(card);
       }
       return None
    }

    pub fn get_right_neighbor(&self, board: &Board, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = board.get_cell(row, column + 1) {
           return Some(card);
       }
       return None
    }

}