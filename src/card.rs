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
}