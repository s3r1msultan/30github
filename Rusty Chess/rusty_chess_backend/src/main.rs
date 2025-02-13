use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::game::ChessGame;

mod game;

#[derive(Serialize)]
struct GameState {
    board_fen: String,
}

#[derive(Deserialize)]
struct MoveRequest {
    chess_move: String,
}

async fn get_board(game: web::Data<Arc<ChessGame>>) -> impl Responder {
    let fen = game.get_board_fen();
    HttpResponse::Ok().json(GameState { board_fen: fen })
}

async fn make_move(game: web::Data<Arc<ChessGame>>, move_req: web::Json<MoveRequest>) -> impl Responder {
    match game.make_move(&move_req.chess_move) {
        Ok(new_fen) => HttpResponse::Ok().json(GameState { board_fen: new_fen }),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chess_game = Arc::new(ChessGame::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(chess_game.clone()))
            .route("/board", web::get().to(get_board))
            .route("/move", web::post().to(make_move))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
