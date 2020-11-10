mod board;
mod piece;

use piece::PieceType;
use piece::Piece;

fn main()
{
    let mut board = board::BoardState::new();

    board.reset();

    board.draw_board();
}




