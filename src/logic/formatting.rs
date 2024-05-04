use core::fmt;

use super::{
    game_loop::Game,
    squares::{Board, Row, TileFill},
};

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.board)
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
        let [l, r, c] = &self.tiles;
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
