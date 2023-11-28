use std::collections::HashMap;
use std::fmt;


use crate::card::Card;
use crate::player::Player;

#[derive(Clone, Debug)]
pub struct Board {
    pub cards: Vec<Option<Card>>,
    pub cell_owner: HashMap<u8, Player>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cards: vec![None; 9], // Use 9 because of 3x3 size
            cell_owner: HashMap::new(),
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
        
        new_cell_owner.insert(index as u8, player.clone());
        new_board.cell_owner = new_cell_owner;

        return new_board;
    }

    pub fn get_cell_owner(&self, x: u8, y: u8) -> Result<Player, String> {
        let index = (x * 3) + y;
        if index >= 9 {
            return Err("Out of bounds".to_string());
        }
        return Ok(self.cell_owner.get(&index).unwrap().clone());
    }

    pub fn set_cell_owner(&self, owner: Player, x: u8, y: u8) -> Board {
        let index = (x * 3) + y;
        let mut new_cell_owner = self.cell_owner.clone();
        new_cell_owner.insert(index, owner);

        return Board {
            cell_owner: new_cell_owner,
            cards: self.cards.clone(),
        };
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for n in 0..2 {
            let first_card = self.get_card_at(0, n).unwrap();
            let second_card = self.get_card_at(1, n).unwrap();
            let third_card = self.get_card_at(2, n).unwrap();

            let first_card_owner = self.get_cell_owner(0, n).unwrap();
            let second_card_owner = self.get_cell_owner(1, n).unwrap();
            let third_card_owner = self.get_cell_owner(2, n).unwrap();


            write!(f, "-------------------------");
        }
        write!(f, "")
    }
}

                    //     let first_card_owner_string = match first_card_owner {
        //         "Player 1" => "X", // TODO
        //         "Player 2" => "0", // TODO
        //         _ => " "
        //     };
        //     let second_card_owner_string = match first_card_owner {
        //         "Player 1" => "X", // TODO
        //         "Player 2" => "0", // TODO
        //         _ => " "
        //     };
        //     let third_card_owner_string = match first_card_owner {
        //         "Player 1" => "X", // TODO
        //         "Player 2" => "0", // TODO
        //         _ => " "
        //     };

        //     let extreme_line = "|    {:02}    |    {:02}    |    {:02}    |";
        //     let middle_line = "| {:02}  O  {:02} | {:02}  O  {:02} | {:02}  O  {:02} |";
        //     write!(f, "-------------------------");

        //     write!(f, "|    {}    |    {}    |    {}    |", 
        //     if let Some(card1) = first_card { format!("{:02}", card1.top)} else { "  ".to_string() },
        //     if let Some(card2) = second_card { format!("{:02}", card2.top)} else { "  ".to_string() },
        //     if let Some(card3) = third_card { format!("{:02}", card3.top)} else { "  ".to_string() }
        //     );

        //     write!(f, "| {:02}  {}  {:02} | {:02}  {}  {:02} | {:02}  {}  {:02} |", 
        //     if let Some(card) = first_card { format!("{:02}", card.left)} else { "  ".to_string() },
        //     first_card_owner_string,
        //     if let Some(card) = first_card { format!("{:02}", card.right)} else { "  ".to_string() },
        //     if let Some(card) = second_card { format!("{:02}", card.left)} else { "  ".to_string() },
        //     second_card_owner_string,
        //     if let Some(card) = second_card { format!("{:02}", card.right)} else { "  ".to_string() },
        //     if let Some(card) = third_card { format!("{:02}", card.left)} else { "  ".to_string() },
        //     third_card_owner_string,
        //     if let Some(card) = third_card { format!("{:02}", card.right)} else { "  ".to_string() }
        //     );

        //     write!(f, "|    {}    |    {}    |    {}    |", 
        //     if let Some(card) = first_card { format!("{:02}", card.bottom)} else { "  ".to_string() },
        //     if let Some(card) = second_card { format!("{:02}", card.bottom)} else { "  ".to_string() },
        //     if let Some(card) = third_card { format!("{:02}", card.bottom)} else { "  ".to_string() }
        //     );

         //}
        //write!(f, "-------------------------");
        
        //}
    //}
//}