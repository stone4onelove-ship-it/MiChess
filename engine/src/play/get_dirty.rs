use super::*;


impl Game {
    pub(super) fn fill_dirty(&mut self) -> () {
        // if not first move
        if let Some(last_mv) = self.played.mv {
            self.played.dirty = BitBoard::new();

            // update move
            self.played.dirty.set(last_mv.mv.0);
            self.played.dirty.set(last_mv.mv.1);
            self.get_dirty_for_pos(last_mv.mv.0);
            self.get_dirty_for_pos(last_mv.mv.1);
            

            // update possible en passant
            if let Some(en_passant) = self.state.en_passant {
                match en_passant {
                    0 => self.played.dirty.set(last_mv.mv.1.row()*8 + last_mv.mv.1.col() + 1),
                    7 => self.played.dirty.set(last_mv.mv.1.row()*8 + last_mv.mv.1.col() - 1),
                    _ => {
                        self.played.dirty.set(last_mv.mv.1.row()*8 + last_mv.mv.1.col() + 1);
                        self.played.dirty.set(last_mv.mv.1.row()*8 + last_mv.mv.1.col() - 1);
                    },
                }
            }

            match last_mv.tp {
                MoveType::EnPassant => {
                    // update captured pawn
                    self.get_dirty_for_pos(last_mv.mv.0.row()*8 + last_mv.mv.1.col());  
                },
                MoveType::Castle(rook_mv) => {
                    self.played.dirty.set(rook_mv.1);
                    self.get_dirty_for_pos(rook_mv.0);
                    self.get_dirty_for_pos(rook_mv.1);
                },
                _ => {}
            }
        } else {
            self.played.dirty.set_all();
        }
    }


    fn get_dirty_for_pos(&mut self, given_pos: Pos) -> () {
        // all who attacks the square
        for pos in 0..64 {
            if self.cache.cover[pos].get(given_pos) {
                self.played.dirty.set(pos);
            }
        }

        // pawns in range of their legal moves
        for row_offset in [1, 2, -1, -2] {
            let pos: i8 = given_pos as i8 + row_offset*8;
            if pos >= 0 && pos < 64 {
                if self.state.board[pos as u8].is_some_and(|p| p.role == Role::Pawn) {
                    self.played.dirty.set(pos as u8);
                }
            }
        }
    }
}