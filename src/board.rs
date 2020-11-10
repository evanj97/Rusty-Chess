use crate::piece;

use piece::PieceType;
use piece::Piece;

use std::collections::HashMap;

pub struct BoardState
{
    board: HashMap<u8, piece::Piece>
}

impl BoardState
{
    pub fn new() -> Self
    {
        BoardState
        {
            board: HashMap::new()
        }
    }

    pub fn reset(&mut self)
    {
        self.board.clear();

        self.board.insert(0, Piece::new(PieceType::Rook, false));
        self.board.insert(1, Piece::new(PieceType::Knight, false));
        self.board.insert(2, Piece::new(PieceType::Bishop, false));
        self.board.insert(3, Piece::new(PieceType::King, false));
        self.board.insert(4, Piece::new(PieceType::Queen, false));
        self.board.insert(5, Piece::new(PieceType::Bishop, false));
        self.board.insert(6, Piece::new(PieceType::Knight, false));
        self.board.insert(7, Piece::new(PieceType::Rook, false));

        for i in 8..16
        {
            self.board.insert(i, Piece::new(PieceType::Pawn, false));
        }


        for i in 48..56
        {
            self.board.insert(i, Piece::new(PieceType::Pawn, true));
        }

        self.board.insert(56, Piece::new(PieceType::Rook, true));
        self.board.insert(57, Piece::new(PieceType::Knight, true));
        self.board.insert(58, Piece::new(PieceType::Bishop, true));
        self.board.insert(59, Piece::new(PieceType::King, true));
        self.board.insert(60, Piece::new(PieceType::Queen, true));
        self.board.insert(61, Piece::new(PieceType::Bishop, true));
        self.board.insert(62, Piece::new(PieceType::Knight, true));
        self.board.insert(63, Piece::new(PieceType::Rook, true));
    }

    pub fn draw_board(self)
    {
        println!("-----------------------------------------");

        for y in 0u8..8
        {
            //println!("#    #    #    #    #    #    #    #    #");

            for x in 0u8..8
            {
                if let Some(icon) = self.board.get(&(x + (8 * y)))
                {
                    print!("| {} ", icon.to_string());
                } else {
                    print!("|    ");
                }
            }
            println!("|");
            //println!("#    #    #    #    #    #    #    #    #");
            println!("-----------------------------------------");
        }
    }
}