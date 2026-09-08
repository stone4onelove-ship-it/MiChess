use super::*;

use std::fmt;


pub struct State {
    // game info
    pub board: Board,
    pub check: bool,
    pub player: Color,
    pub mode: GameMode,
    pub legal: BitGrid,
    // for dynamic update
    pub played: Option<PlayedMove>,
    // history
    pub history: Vec<GameLog>,
}



impl Game {
    pub fn state(&self) -> State {
        State {
            // game info
            board: self.board.clone(),
            check: self.check,
            player: self.player,
            mode: self.mode,
            legal: self.legal,
            played: self.played,
            history: self.history.clone(),
        }
    }
}


impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n\n   a  b  c  d  e  f  g  h     |")?;

        for pos in 0..64 {

            if pos % 8 == 0 { 
                write!(f, " {} ", (8 - (pos / 8)))?;
            }

            match self.board[pos] {
                Some(piece) => write!(f, "{} ", piece)?,
                None => write!(f, "__ ")?,
            }

            if pos % 8 == 7 {
                match pos / 8 {
                    0 => writeln!(f, "8  |  Player to move: {}", self.player)?,
                    1 => writeln!(f, "7  |")?,
                    2 => writeln!(f, "6  |  Castle: {:?}", self.castle)?,
                    3 => writeln!(f, "5  |  En passant: {:?}", self.en_passant)?,
                    4 => writeln!(f, "4  |  50 moves rule: {}", self.rule_50moves)?,
                    5 => writeln!(f, "3  |  Check: {}", self.check)?,
                    6 => writeln!(f, "2  |  Moves played: {}", self.history.len())?,
                    7 => writeln!(f, "1  |  Last played move: {:?}", self.played)?,
                    _ => {},
                }
            }
        }

        writeln!(f, "   a  b  c  d  e  f  g  h     |\n")?;
        writeln!(f, "      Cover: White                 Cover: Black                 Updated Pos")?;


        for rank in 0..8 {
            // cover White
            for file in 0..8 {
                let pos = rank * 8 + file;
                match self.cover_comb[0].get(pos) {
                    true => write!(f, "XX ")?,
                    false => write!(f, "__ ")?,
                }
            }
            write!(f, "     ")?;

            // cover Balck
            for file in 0..8 {
                let pos = rank * 8 + file;
                match self.cover_comb[1].get(pos) {
                    true => write!(f, "XX ")?,
                    false => write!(f, "__ ")?,
                }
                
            }
            write!(f, "     ")?;

            // dirty moves
            for file in 0..8 {
                let pos = rank * 8 + file;
                match self.dirty.get(pos) {
                    true => write!(f, "XX ")?,
                    false => write!(f, "__ ")?,
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "\ntransformer White: {:?}", self.transformer.0)?;
        writeln!(f, "transformer Black: {:?}", self.transformer.1)?;

        writeln!(f,"\n")
    }
}