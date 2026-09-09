use super::*;

impl Game {
    pub(super) fn update_cover_comb(&mut self) -> () {
        self.cache.cover_comb = [BitBoard::new(), BitBoard::new()];

        for pos in 0..64 {
            if let Some(piece) = self.state.board[pos] {
                self.cache.cover_comb[piece.color as usize].0 |= self.cache.cover[pos].0;
            }
        }
    }
}