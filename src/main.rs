enum Commands {
    Register {name: String},
    Select {card: Card},
    ChoosePlayer,
    Play {card: Card, x: u8, y: u8},
    Check {x: u8, y: u8},
}

struct Card {
    id: String,
    owner: u8,
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
}

struct Player {
    id: u8,
    score: u32,
    name: String,
    hand: Vec<Card>,
}

struct Board {
    cards: Vec<Vec<Option<Card>>>,
}

struct GameController {

}

trait Rule {
}


fn main() {
    println!("Hello, world!");
}