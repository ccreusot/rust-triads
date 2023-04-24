#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Owner {
    PlayerOne,
    PlayerTwo
}

impl Owner {
    pub fn to_sign(self) -> String {
        match self {
            Owner::PlayerOne => "o".to_string(),
            Owner::PlayerTwo => "x".to_string()
        }
    }
}