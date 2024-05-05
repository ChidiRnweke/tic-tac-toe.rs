use ansi_term::Colour::{Blue, Red};
use core::fmt;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileFill {
    X,
    O,
    Empty,
}

impl TileFill {
    pub fn get_next_player(self) -> Self {
        match self {
            Self::O => Self::X,
            Self::X => Self::O,
            Self::Empty => panic!("The current player cannot be empty."),
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum RowTarget {
    Bottom,
    Center,
    Top,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum ColumnTarget {
    Left,
    Center,
    Right,
}

impl TryFrom<usize> for RowTarget {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::Top),
            2 => Ok(Self::Center),
            1 => Ok(Self::Bottom),
            _ => Err("An invalid move was given. The input must be 2 for Top, 1 for Center and 0 for bottom."),
        }
    }
}

impl From<&RowTarget> for usize {
    fn from(value: &RowTarget) -> Self {
        match value {
            RowTarget::Top => 2,
            RowTarget::Center => 1,
            RowTarget::Bottom => 0,
        }
    }
}

impl TryFrom<usize> for ColumnTarget {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::Right),
            2 => Ok(Self::Center),
            1 => Ok(Self::Left),
            _ => Err("An invalid move was given. The input must be 3 for Right, 2 for Center and 1 for left."),
        }
    }
}

impl From<&ColumnTarget> for usize {
    fn from(value: &ColumnTarget) -> Self {
        match value {
            ColumnTarget::Right => 2,
            ColumnTarget::Center => 1,
            ColumnTarget::Left => 0,
        }
    }
}

pub struct ValidMove {
    row: RowTarget,
    col: ColumnTarget,
}

impl ValidMove {
    pub fn new(board: &Board, row_target: RowTarget, col_target: ColumnTarget) -> Option<Self> {
        let row_num = usize::from(&row_target);
        let col_num: usize = usize::from(&col_target);
        let row = &board.rows[row_num];
        let target_tile = &row.tiles[col_num];

        if *target_tile == TileFill::Empty {
            Some(Self {
                row: row_target,
                col: col_target,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Row {
    tiles: [TileFill; 3],
}

struct RowReference<'a> {
    tiles: [&'a TileFill; 3],
}

impl Row {
    const fn new() -> Self {
        Self {
            tiles: [TileFill::Empty; 3],
        }
    }

    fn is_complete(&self) -> bool {
        self.tiles.iter().all(|&tile| tile == TileFill::X)
            || self.tiles.iter().all(|&tile| tile == TileFill::O)
    }

    fn is_not_full(&self) -> bool {
        self.tiles.iter().any(|tile| *tile == TileFill::Empty)
    }
}

impl RowReference<'_> {
    fn is_complete(&self) -> bool {
        self.tiles.iter().all(|&tile| *tile == TileFill::X)
            || self.tiles.iter().all(|&tile| *tile == TileFill::O)
    }
}

#[derive(Clone)]
pub struct Board {
    rows: [Row; 3],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            rows: [Row::new(), Row::new(), Row::new()],
        }
    }
}
impl Board {
    pub fn make_move(&self, player: TileFill, player_move: &ValidMove) -> Self {
        let row_num = usize::from(&player_move.row);
        let col_num: usize = usize::from(&player_move.col);
        let mut new_board = self.clone();
        new_board.rows[row_num].tiles[col_num] = player;
        new_board
    }

    pub fn is_complete(&self) -> bool {
        self.any_row_complete() || self.any_diagonal_complete() || self.any_vertical_complete()
    }

    pub fn is_draw(&self) -> bool {
        !self.rows.iter().any(Row::is_not_full)
    }

    fn any_row_complete(&self) -> bool {
        self.rows.iter().any(Row::is_complete)
    }

    fn any_diagonal_complete(&self) -> bool {
        let (diag1, diag2) = self.make_diagonals();
        diag1.is_complete() || diag2.is_complete()
    }

    fn any_vertical_complete(&self) -> bool {
        let (vert1, vert2, vert3) = self.make_verticals();
        vert1.is_complete() || vert2.is_complete() || vert3.is_complete()
    }

    const fn make_diagonals(&self) -> (RowReference, RowReference) {
        let diag_1 = RowReference {
            tiles: [
                &self.rows[0].tiles[0],
                &self.rows[1].tiles[1],
                &self.rows[2].tiles[2],
            ],
        };

        let diag_2 = RowReference {
            tiles: [
                &self.rows[0].tiles[2],
                &self.rows[1].tiles[1],
                &self.rows[2].tiles[0],
            ],
        };
        (diag_1, diag_2)
    }

    const fn make_verticals(&self) -> (RowReference, RowReference, RowReference) {
        let vert_1 = RowReference {
            tiles: [
                &self.rows[0].tiles[0],
                &self.rows[1].tiles[0],
                &self.rows[2].tiles[0],
            ],
        };
        let vert_2 = RowReference {
            tiles: [
                &self.rows[0].tiles[1],
                &self.rows[1].tiles[1],
                &self.rows[2].tiles[1],
            ],
        };

        let vert_3 = RowReference {
            tiles: [
                &self.rows[0].tiles[2],
                &self.rows[1].tiles[2],
                &self.rows[2].tiles[2],
            ],
        };
        (vert_1, vert_2, vert_3)
    }
}

impl fmt::Display for TileFill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let o = &Red.paint("O").to_string();
        let x = &Blue.paint("X").to_string();

        let string_rep = match self {
            Self::Empty => " ",
            Self::O => o,
            Self::X => x,
        };
        write!(f, "{string_rep}")
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [l, c, r] = &self.tiles;
        write!(f, "|{l}|{c}|{r}|")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [b, c, t] = &self.rows;
        write!(
            f,
            "
  ------
3 {t}
  ------
2 {c}
  ------
1 {b}
  ------
   1 2 3"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_player_should_correctly_be_set() {
        assert_eq!(TileFill::X.get_next_player(), TileFill::O);
        assert_eq!(TileFill::O.get_next_player(), TileFill::X);
    }

    #[test]
    fn row_numbers_are_mapped_to_correct_target() {
        assert_eq!(RowTarget::try_from(3).unwrap(), RowTarget::Top);
        assert_eq!(RowTarget::try_from(2).unwrap(), RowTarget::Center);
        assert_eq!(RowTarget::try_from(1).unwrap(), RowTarget::Bottom);
    }

    #[test]
    fn incorrect_row_numbers_should_panic() {
        assert!(RowTarget::try_from(0).is_err());
    }

    #[test]
    fn column_numbers_are_mapped_to_correct_target() {
        assert_eq!(ColumnTarget::try_from(3).unwrap(), ColumnTarget::Right);
        assert_eq!(ColumnTarget::try_from(2).unwrap(), ColumnTarget::Center);
        assert_eq!(ColumnTarget::try_from(1).unwrap(), ColumnTarget::Left);
    }

    #[test]
    fn incorrect_col_number_should_panic() {
        assert!(ColumnTarget::try_from(0).is_err());
    }

    #[test]
    fn making_a_move_on_empty_tile_is_valid() {
        let board = Board::default();
        let valid_move = ValidMove::new(&board, RowTarget::Bottom, ColumnTarget::Left);
        assert!(valid_move.is_some());
    }

    #[test]
    fn making_a_move_on_nonempty_tile_is_invalid() {
        let board = Board::default();
        let valid = ValidMove::new(&board, RowTarget::Top, ColumnTarget::Right).unwrap();
        let new_board = board.make_move(TileFill::X, &valid);
        let invalid_move = ValidMove::new(&new_board, RowTarget::Top, ColumnTarget::Right);
        assert!(invalid_move.is_none());
    }

    #[test]
    fn moves_should_be_made_correctly() {
        let board = Board::default();
        let valid_move = ValidMove {
            row: RowTarget::try_from(1).unwrap(),
            col: ColumnTarget::try_from(1).unwrap(),
        };
        let updated_board = board.make_move(TileFill::X, &valid_move);
        assert_eq!(updated_board.rows[0].tiles[0], TileFill::X);
    }

    #[test]
    fn test_board_horizontal_complete() {
        let mut board = Board::default();
        board.rows[0].tiles = [TileFill::X, TileFill::X, TileFill::X];
        assert!(board.is_complete());
    }

    #[test]
    fn test_board_vertical_complete() {
        let mut board = Board::default();
        board.rows[0].tiles[0] = TileFill::X;
        board.rows[0].tiles[1] = TileFill::X;
        board.rows[0].tiles[2] = TileFill::X;
        assert!(board.is_complete());
    }

    #[test]
    fn test_board_diagonal1_complete() {
        let mut board = Board::default();
        board.rows[0].tiles[0] = TileFill::X;
        board.rows[1].tiles[1] = TileFill::X;
        board.rows[2].tiles[2] = TileFill::X;
        assert!(board.is_complete());
    }

    #[test]
    fn test_board_diagonal2_complete() {
        let mut board = Board::default();
        board.rows[0].tiles[2] = TileFill::X;
        board.rows[1].tiles[1] = TileFill::X;
        board.rows[2].tiles[0] = TileFill::X;
        assert!(board.is_complete());
    }

    #[test]
    fn test_board_is_draw() {
        let mut board = Board::default();
        for row in board.rows.iter_mut() {
            row.tiles = [TileFill::X, TileFill::O, TileFill::X];
        }
        assert!(board.is_draw());
    }
}
