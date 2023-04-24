use crate::owner::Owner;
use crate::strength::Strength;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub owner: Owner,
    pub top: Strength,
    pub left: Strength,
    pub bottom: Strength,
    pub right: Strength
}

impl Card {
    pub fn to_lines(&self) -> [String; 3] {
        [
            format!("   {}   |", self.top.to_value()),
            format!("{}  {}  {}|", self.left.to_value(), self.owner.to_sign(), self.right.to_value()),
            format!("   {}   |", self.bottom.to_value())
        ]
    }
}
