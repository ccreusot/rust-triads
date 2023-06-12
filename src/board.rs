use crate::card::Card;
use crate::cell::Cell;

#[derive(Clone, Copy)]
pub struct Board {
    cells: [Cell; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<Cell> {
        if row > 2 || column > 2 {
            return None;
        }

        if row < 0 || column < 0 {
            return None;
        }
        return Some(self.cells[(row * 3) + column])
    }

    pub fn get_bottom_neighbor(&self, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = self.get_cell(row + 1, column) {
           return Some(card);
       }
       return None
    }

    pub fn get_top_neighbor(&self, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = self.get_cell(row - 1, column) {
           return Some(card);
       }
       return None
    }

    pub fn get_left_neighbor(&self, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = self.get_cell(row, column - 1) {
           return Some(card);
       }
       return None
    }

    pub fn get_right_neighbor(&self, row: usize, column: usize) -> Option<Card> {
       if let Some(Cell::Card { card }) = self.get_cell(row, column + 1) {
           return Some(card);
       }
       return None
    }

    pub fn place_card(&self, row: usize, column: usize, card: Card) -> Board {
        let mut board_clone = self.clone();
        board_clone.cells[(row * 3) + column] = Cell::Card { card: card };
        board_clone
    }

    pub fn display(&self) -> String {
        let line_separator = "  -------------------------\n";
        let last_line = "      A       B       C\n";
        let mut board = "\n".to_string();
        board.push_str(line_separator);

        for row in 0..3 {
            let mut line: [String; 3] = [
                "  |".to_string(),
                format!("{} |", 3 - row),
                "  |".to_string(),
            ];
            for column in 0..3 {
                let cell = self.cells[(row * 3) + column].to_lines();
                line[0].push_str(cell[0].as_str());
                line[1].push_str(cell[1].as_str());
                line[2].push_str(cell[2].as_str());
            }
            board.push_str(line.join("\n").as_str());
            board.push_str("\n");
            board.push_str(line_separator);
        }
        board.push_str(last_line);
        println!("{}", board);
        return board;
    }
}

// Attribute used to add metadata or to apply a crates here it is the test crate
// to let us write unit test.
#[cfg(test)]
mod board_tests {
    use crate::board::Board;
    use crate::card::Card;
    use crate::cell::Cell;
    use crate::owner::Owner;
    use crate::strength::Strength;

    #[test]
    fn test_empty_board() {
        let expected = "
  -------------------------
  |       |       |       |
3 |       |       |       |
  |       |       |       |
  -------------------------
  |       |       |       |
2 |       |       |       |
  |       |       |       |
  -------------------------
  |       |       |       |
1 |       |       |       |
  |       |       |       |
  -------------------------
      A       B       C
";
        assert_eq!(expected, Board::new().display());
    }

    #[test]
    fn test_display_board_filled_with_one_card() {
        let expected = "
  -------------------------
  |       |       |       |
3 |       |       |       |
  |       |       |       |
  -------------------------
  |       |   2   |       |
2 |       |2  o  2|       |
  |       |   2   |       |
  -------------------------
  |       |       |       |
1 |       |       |       |
  |       |       |       |
  -------------------------
      A       B       C
";
        let board = Board::new().place_card(
            1,
            1,
            Card {
                owner: Owner::PlayerOne,
                top: Strength::Two,
                left: Strength::Two,
                bottom: Strength::Two,
                right: Strength::Two,
            },
        );
        assert_eq!(expected, board.display());
    }

    #[test]
    fn test_display_board_filled_with_two_card() {
        let expected = "
  -------------------------
  |       |       |       |
3 |       |       |       |
  |       |       |       |
  -------------------------
  |       |   2   |       |
2 |       |1  o  4|       |
  |       |   3   |       |
  -------------------------
  |   5   |       |       |
1 |8  x  6|       |       |
  |   A   |       |       |
  -------------------------
      A       B       C
";

        let board = Board::new()
            .place_card(
                1,
                1,
                Card {
                    owner: Owner::PlayerOne,
                    top: Strength::Two,
                    left: Strength::One,
                    bottom: Strength::Three,
                    right: Strength::Four,
                },
            )
            .place_card(
                2,
                0,
                Card {
                    owner: Owner::PlayerTwo,
                    top: Strength::Five,
                    left: Strength::Eight,
                    bottom: Strength::A,
                    right: Strength::Six,
                },
            );
        assert_eq!(expected, board.display());
    }

}
