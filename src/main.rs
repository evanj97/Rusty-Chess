mod board;
mod piece;

use piece::PieceType;
use piece::Piece;
use std::io;

fn main()
{
    let mut board = board::BoardState::new();

    board.reset();
    board.draw_board();

    loop
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line"); // read input

        let mut split = input.split(' '); // split and parse

        loop
        {
            let mut o_token = split.next();

            if let Some(token) = o_token
            {

            }
        }
    }
}

