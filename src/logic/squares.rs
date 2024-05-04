use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileFill {
    X,
    O,
    Empty,
}

impl TileFill {
    pub fn get_next_player(&self) -> TileFill {
        match self {
            TileFill::O => TileFill::X,
            TileFill::X => TileFill::O,
            TileFill::Empty => panic!("The current player cannot be empty."),
        }
    }
}

impl fmt::Display for TileFill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_rep = match self {
            TileFill::Empty => " ",
            TileFill::O => "O",
            TileFill::X => "X",
        };
        write!(f, "{string_rep}")
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = &self.tiles[0];
        let c = &self.tiles[1];
        let r = &self.tiles[2];
        write!(f, "|{l}|{c}|{r}|")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = &self.rows[0];
        let c = &self.rows[1];
        let t = &self.rows[2];
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

pub enum RowTarget {
    Bottom,
    Center,
    Top,
}

pub enum ColumnTarget {
    Left,
    Center,
    Right,
}

impl TryFrom<usize> for RowTarget {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(RowTarget::Top),
            2 => Ok(RowTarget::Center),
            1 => Ok(RowTarget::Bottom),
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
            3 => Ok(ColumnTarget::Right),
            2 => Ok(ColumnTarget::Center),
            1 => Ok(ColumnTarget::Left),
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
    pub fn new(
        board: &Board,
        row_target: RowTarget,
        col_target: ColumnTarget,
    ) -> Option<ValidMove> {
        let row_num = usize::from(&row_target);
        let col_num: usize = usize::from(&col_target);
        let row = &board.rows[row_num];
        let col = &row.tiles[col_num];

        if let TileFill::Empty = col {
            Some(ValidMove {
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

struct Diagonal<'a> {
    tiles: [&'a TileFill; 3],
}

impl Row {
    fn new() -> Self {
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

impl Diagonal<'_> {
    fn is_complete(&self) -> bool {
        self.tiles.iter().all(|&tile| *tile == TileFill::X)
            || self.tiles.iter().all(|&tile| *tile == TileFill::O)
    }
}

#[derive(Clone)]
pub struct Board {
    rows: [Row; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: [Row::new(), Row::new(), Row::new()],
        }
    }

    pub fn make_move(&self, player: TileFill, player_move: ValidMove) -> Board {
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
        !self.rows.iter().any(|row| row.is_not_full())
    }

    fn any_row_complete(&self) -> bool {
        self.rows.iter().any(|row| row.is_complete())
    }

    fn any_diagonal_complete(&self) -> bool {
        let (diag1, diag2) = self.make_diagonals();
        diag1.is_complete() || diag2.is_complete()
    }

    fn any_vertical_complete(&self) -> bool {
        let (vert1, vert2, vert3) = self.make_verticals();
        vert1.is_complete() || vert2.is_complete() || vert3.is_complete()
    }

    fn make_diagonals(&self) -> (Diagonal, Diagonal) {
        let diag_1 = Diagonal {
            tiles: [
                &self.rows[0].tiles[0],
                &self.rows[1].tiles[1],
                &self.rows[2].tiles[2],
            ],
        };

        let diag_2 = Diagonal {
            tiles: [
                &self.rows[0].tiles[2],
                &self.rows[1].tiles[1],
                &self.rows[2].tiles[0],
            ],
        };
        (diag_1, diag_2)
    }

    fn make_verticals(&self) -> (Diagonal, Diagonal, Diagonal) {
        let vert_1 = Diagonal {
            tiles: [
                &self.rows[0].tiles[0],
                &self.rows[1].tiles[0],
                &self.rows[2].tiles[0],
            ],
        };
        let vert_2 = Diagonal {
            tiles: [
                &self.rows[0].tiles[1],
                &self.rows[1].tiles[1],
                &self.rows[2].tiles[1],
            ],
        };

        let vert_3 = Diagonal {
            tiles: [
                &self.rows[0].tiles[2],
                &self.rows[1].tiles[2],
                &self.rows[2].tiles[2],
            ],
        };
        (vert_1, vert_2, vert_3)
    }
}
