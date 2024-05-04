#[derive(Clone, Copy, PartialEq)]
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

pub enum RowTarget {
    Top,
    Center,
    Bottom,
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
            2 => Ok(RowTarget::Top),
            1 => Ok(RowTarget::Center),
            0 => Ok(RowTarget::Bottom),
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
            2 => Ok(ColumnTarget::Right),
            1 => Ok(ColumnTarget::Center),
            0 => Ok(ColumnTarget::Left),
            _ => Err("An invalid move was given. The input must be 2 for Right, 1 for Center and 0 for left."),
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

pub struct ValidMove<'a> {
    row: &'a RowTarget,
    col: &'a ColumnTarget,
}

impl<'a> ValidMove<'a> {
    pub fn new(
        board: &Board,
        row_target: &'a RowTarget,
        col_target: &'a ColumnTarget,
    ) -> Option<ValidMove<'a>> {
        let row_num: usize = row_target.into();
        let col_num: usize = col_target.into();
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
        let col: usize = player_move.col.into();
        let row: usize = player_move.row.into();
        let mut new_board = self.clone();
        new_board.rows[row].tiles[col] = player;
        new_board
    }

    pub fn is_complete(&self) -> bool {
        self.any_row_complete() || self.any_diagonal_complete()
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
}
