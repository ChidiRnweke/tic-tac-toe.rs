use core::fmt;

use super::{
    game_loop::Game,
    squares::{Board, Row, TileFill},
};

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = &self.board;
        write!(f, "{board}")
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
