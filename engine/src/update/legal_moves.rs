use super::*;

impl Game { 
    pub(super) fn update_legal_moves(&mut self) {
        // clear 
        self.cache.legal_moves.clear();
        // fill
        for start_pos in 0..64 {
            if self.state.board[start_pos].is_some_and(|p| p.color == self.state.player) {
                let mut bits = self.cache.legal[start_pos].0;
                while bits != 0 {
                    let end_pos = bits.trailing_zeros() as u8;
                    self.cache.legal_moves.push((start_pos, end_pos));
                    bits &= bits - 1;  // clear lowest set bit
                }
            }
        }
    }
}