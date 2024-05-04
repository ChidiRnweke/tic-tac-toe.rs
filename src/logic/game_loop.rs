use super::squares::{Board, TileFill, ValidMove};

pub struct Game {
    board: Board,
    current_player: TileFill,
}

impl Game {
    pub fn start_game() -> Game {
        Game {
            board: Board::new(),
            current_player: TileFill::O,
        }
    }

    pub fn make_move(&self, valid_move: ValidMove) -> Game {
        let next_turn = self.board.make_move(self.current_player, valid_move);
        Game {
            board: next_turn,
            current_player: self.current_player.get_next_player(),
        }
    }
}
