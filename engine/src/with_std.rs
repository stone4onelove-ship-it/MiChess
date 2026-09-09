use super::*;

use std::fmt;


impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                let bit = (self.0 >> (row * 8 + col)) & 1;
                write!(f, "{} ", bit)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "W"),
            Color::Black => write!(f, "B"),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Pawn   => write!(f, "P"),
            Role::Knight => write!(f, "H"),
            Role::Bishop => write!(f, "B"),
            Role::Rook   => write!(f, "R"),
            Role::Queen  => write!(f, "Q"),
            Role::King   => write!(f, "K"),
        }
    }
}


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.role)
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, square) in self.0.iter().enumerate() {
            match square {
                Some(piece) => write!(f, "{} ", piece)?,
                None => write!(f, "__ ")?,
            }
            if (i + 1) % 8 == 0 {
                writeln!(f)?;
            }
        }
        writeln!(f)?;
        Ok(())
    }
}


impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n\n   a  b  c  d  e  f  g  h     |")?;

        for pos in 0..64 {

            if pos % 8 == 0 { 
                write!(f, " {} ", (8 - (pos / 8)))?;
            }

            match self.state.board[pos] {
                Some(piece) => write!(f, "{} ", piece)?,
                None => write!(f, "__ ")?,
            }

            if pos % 8 == 7 {
                match pos / 8 {
                    0 => writeln!(f, "8  |  Player to move: {}", self.state.player)?,
                    1 => writeln!(f, "7  |")?,
                    2 => writeln!(f, "6  |  Castle: {:?}", self.state.castle)?,
                    3 => writeln!(f, "5  |  En passant: {:?}", self.state.en_passant)?,
                    4 => writeln!(f, "4  |  50 moves rule: {}", self.state.rule_50moves)?,
                    5 => writeln!(f, "3  |  Check: {}", self.cache.check)?,
                    6 => writeln!(f, "2  |  Moves played: {}", self.history.len())?,
                    7 => writeln!(f, "1  |  Last played move: {:?}", self.played.mv)?,
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
                match self.cache.cover_comb[0].get(pos) {
                    true => write!(f, "XX ")?,
                    false => write!(f, "__ ")?,
                }
            }
            write!(f, "     ")?;

            // cover Balck
            for file in 0..8 {
                let pos = rank * 8 + file;
                match self.cache.cover_comb[1].get(pos) {
                    true => write!(f, "XX ")?,
                    false => write!(f, "__ ")?,
                }
                
            }
            write!(f, "     ")?;

            // dirty moves
            for file in 0..8 {
                let pos = rank * 8 + file;
                match self.played.dirty.get(pos) {
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