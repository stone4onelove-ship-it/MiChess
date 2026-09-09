mod piece;
pub use piece::*;
mod board;
pub use board::Board;
mod bitgrid;
pub use bitgrid::BitGrid;
mod bitboard;
pub use bitboard::BitBoard;


pub type Pos = u8;

pub trait PosExt {
    fn row(self) -> u8;
    fn col(self) -> u8;
}

impl PosExt for u8 {
    fn row(self) -> u8 { self / 8 }
    fn col(self) -> u8 { self % 8 }
}


pub type Move = (Pos, Pos);


#[derive(Copy, Clone, Debug)]
pub struct PlayedMove {
    pub mv: Move,
    pub tp: MoveType,
    pub captured: Option<Piece>,

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveType {
    Basic,
    Promotion,
    Castle(Move),
    EnPassant,
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveError {
    WrongMode,
    NoPiece,
    IlligalMove,
    KingInDanger,
}
    

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameMode {
    Active,
    Finished(Option<Color>),
}