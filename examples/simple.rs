extern crate warp_square_engine;

use warp_square_engine::game::Game;

fn main() {
    let game = Game::new();

    game.board.pieces.iter().for_each(|piece| {
        println!(
            "{:?}({:?}): {:?}",
            piece.piece_type,
            piece.color,
            piece.get_attack_squares(&game.board)
        );
    });
}
