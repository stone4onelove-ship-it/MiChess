use super::*;


#[derive(Clone)]
pub struct Board( pub [Option<Piece>; 64] );

impl Board {
    pub fn new() -> Self {
        Board([
            BR, BH, BB, BQ, BK, BB, BH, BR,
            BP, BP, BP, BP, BP, BP, BP, BP,
            __, __, __, __, __, __, __, __,
            __, __, __, __, __, __, __, __,
            __, __, __, __, __, __, __, __,
            __, __, __, __, __, __, __, __,
            WP, WP, WP, WP, WP, WP, WP, WP,
            WR, WH, WB, WQ, WK, WB, WH, WR,
        ])
    } 

    pub fn king_pos(&self) -> [u8; 2] {
        let mut king_pos = [64;2];
        for pos in 0..64 {
            match self[pos] {
                Some(Piece {color: Color::White, role: Role::King}) => {
                    if king_pos[0] != 64 {panic!("Too many white kings.")}

                    king_pos[0] = pos;
                },
                Some(Piece {color: Color::Black, role: Role::King}) => {
                    if king_pos[1] != 64 {panic!("Too many black kings.")}

                    king_pos[1] = pos;
                },
                _ => {},
            }
        };

        if king_pos[0] == 64 { panic!("Missing white king.") }
        if king_pos[1] == 64 { panic!("Missing black king.") }
        

        king_pos
    }
}



impl std::ops::Index<Pos> for Board {
    type Output = Option<Piece>;
    fn index(&self, pos: Pos) -> &Self::Output {
        &self.0[pos as usize]
    }
}

impl std::ops::IndexMut<Pos> for Board {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.0[pos as usize]
    }
}

impl std::fmt::Display for Board {
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
