#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, AppHandle, Manager, State};
use std::sync::Mutex;
use chess::{Board, ChessMove, Game, MoveGen};
use std::str::FromStr;

struct ChessGameState {
  game: Mutex<Game>,
}

impl ChessGameState {
  fn new() -> Self {
    Self {
      game: Mutex::new(Game::new()),
    }
  }

  fn get_board_fen(&self) -> String {
    let game = self.game.lock().unwrap();
    game.current_position().to_string()
  }

  fn make_move(&self, move_str: String) -> Result<String, String> {
    let mut game = self.game.lock().unwrap();
    let chess_move = ChessMove::from_str(&move_str)
        .map_err(|_| "Invalid move format".to_string())?;

    if game.make_move(chess_move) {
      Ok(game.current_position().to_string())
    } else {
      Err("Illegal move".to_string())
    }
  }
}

#[tauri::command]
fn get_board(state: State<ChessGameState>) -> String {
  state.get_board_fen()
}

#[tauri::command]
fn make_move(state: State<ChessGameState>, move_str: String) -> Result<String, String> {
  state.make_move(move_str)
}

fn main() {
  tauri::Builder::default()
      .manage(ChessGameState::new())
      .invoke_handler(generate_handler![get_board, make_move])
      .run(tauri::generate_context!())
      .expect("error while running Tauri application");
}
