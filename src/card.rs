use crate::owner::Owner;
use crate::strength::Strength;
use crate::board::Board;
use crate::cell::Cell;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
            format!("   {}   |", self.top.to_value()),
            format!(
                "{}  {}  {}|",
                self.left.to_value(),
                self.owner.to_sign(),
                self.right.to_value()
            ),
            format!("   {}   |", self.bottom.to_value()),
        ]
    }

    pub fn get_bottom_neighbor(board: &Board, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = board.get_cell(row + 1, column) {
           return Some(card);
       }
       return None
    }

    pub fn get_top_neighbor(board: &Board, row: usize, column: usize) -> Option<Card> {
        board.get_cell(row - 1, column)
    }

    pub fn get_left_neighbor(board: &Board, row: usize, column: usize) -> Option<Card> {
        board.get_cell(row, column - 1)
    }

    pub fn get_right_neighbor(board: &Board, row: usize, column: usize) -> Option<Card> {
        board.get_cell(row, column + 1)
    }

}