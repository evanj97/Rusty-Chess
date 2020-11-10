#[derive(Copy, Clone)]
pub enum PieceType
{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Empty,
}

#[derive(Copy, Clone)]
pub struct Piece
{
    piece_type: PieceType,
    player: bool,
}

impl Piece
{
    pub fn new(piece_type: PieceType, player: bool) -> Self {
        Piece { piece_type, player }
    }


    pub fn to_string(self) -> String
    {
        if !self.player
        {
            return match self.piece_type
            {
                PieceType::Pawn => "1P".to_string(),
                PieceType::Rook => "1R".to_string(),
                PieceType::Knight => "1N".to_string(),
                PieceType::Bishop => "1B".to_string(),
                PieceType::Queen => "1Q".to_string(),
                PieceType::King => "1K".to_string(),
                PieceType::Empty => " ".to_string(),
            };
        } else {
            return match self.piece_type
            {
                PieceType::Pawn => "2P︎".to_string(),
                PieceType::Rook => "2R".to_string(),
                PieceType::Knight => "2N".to_string(),
                PieceType::Bishop => "2B".to_string(),
                PieceType::Queen => "2Q".to_string(),
                PieceType::King => "2K".to_string(),
                PieceType::Empty => " ".to_string(),
            };
        }
    }
}