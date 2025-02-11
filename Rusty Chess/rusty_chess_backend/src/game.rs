use chess::{Board, ChessMove, Game, GameResult, MoveGen, Square};
use std::sync::Mutex;
use std::str::FromStr;

pub struct ChessGame {
    pub game: Mutex<Game>,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            game: Mutex::new(Game::new()),
        }
    }

    pub fn get_board_fen(&self) -> String {
        let game = self.game.lock().unwrap();
        game.current_position().to_string()
    }

    pub fn make_move(&self, move_str: &str) -> Result<String, String> {
        let mut game = self.game.lock().unwrap();

        let chess_move = ChessMove::from_str(move_str)
            .map_err(|_| "Invalid move format".to_string())?;

        if game.make_move(chess_move) {
            Ok(game.current_position().to_string())
        } else {
            Err("Illegal move".to_string())
        }
    }

    pub fn get_result(&self) -> Option<GameResult> {
        let game = self.game.lock().unwrap();
        game.result()
    }
}
