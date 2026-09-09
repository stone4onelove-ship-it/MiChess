use super::*;

mod get_dirty;
mod mode_checks;
mod make_move;


impl Game {
    // makes a move, returns bool
    pub fn play(&mut self, mv: Move) -> Result<(), MoveError> {
        let (start_pos, end_pos) = mv;
        // checks if the move is legal
        if self.state.mode != GameMode::Active {
            return Err(MoveError::WrongMode);
        }
        if !self.state.board[start_pos].is_some_and(|p| p.color == self.state.player) {
            return Err(MoveError::NoPiece);
        }
        if !self.cache.legal[start_pos].get(end_pos) {
            return Err(MoveError::IlligalMove);
        };
        
        // save history
        self.history.push(self.save());

        // play move
        self.make_move(mv);

        // update cache
        self.update();

        // update transformer
        self.transformer.play(&NNUE, &self.state.board, self.cache.king_pos, self.played.mv.unwrap());

        // check if move is legal, if not loads back up 
        if self.cache.cover_comb[self.state.player as usize].get(self.cache.king_pos[self.state.player.opp() as usize]) {
            self.undo();
            return Err(MoveError::KingInDanger);
        }
        
        // check for wins and draws
        self.mode_check();
        
        return Ok(());
    }
}





