use super::*;


impl Game {
    pub(super) fn mode_check(&mut self) -> () {
        self.win_check();
        self.stalemate_check();
        self.no_material_check();
        self.rule_50_check();
    }


    fn win_check(&mut self) -> () {
        if !self.cache.check {
            return;
        }

        let king_pos = self.cache.king_pos[self.state.player as usize];
        if !self.cache.legal[king_pos].is_empty() {
            return;
        }

        for mv in self.cache.legal_moves.clone() {
            // make move
            self.history.push(self.save());
            self.make_move(mv);
            self.update();
            self.transformer.play(&NNUE, &self.state.board, self.cache.king_pos, self.played.mv.unwrap());
            // if there is any move to avoid mate
            if !self.cache.cover_comb[self.state.player.opp() as usize].get(king_pos) {
                self.undo();
                return;
            }
            self.undo();
        }
        self.state.mode = GameMode::Finished(Some(self.state.player.opp()));
    }


    fn stalemate_check(&mut self) -> () {
        // stalemate
        if !self.cache.check {
            for pos in 0..64 {
                if self.state.board[pos].is_some_and(|p| p.color == self.state.player) &&
                !self.cache.legal[pos].is_empty() {
                    return;
                }
            }
            self.state.mode = GameMode::Finished(None);
            return;
        }
    }


    fn no_material_check(&mut self) -> () {

        let mut w_material: u8 = 0;
        let mut b_material: u8 = 0;

        for pos in 0..64 {
            match self.state.board[pos] {
                WB | WH => w_material += 1,
                BB | BH => b_material += 1,
                WK | BK | __ => (),
                _         =>  { return; }
            }
        }
        if w_material == 0 && b_material <= 1 || w_material <= 1 && b_material == 0 {
            self.state.mode = GameMode::Finished(None);
            return;
        }
    }

    fn rule_50_check(&mut self) {
        // reset
        if let Some(played) = self.played.mv {
            if self.state.board[played.mv.1].is_some_and( |p| p.role == Role::Pawn) && played.captured.is_some() {
                self.state.rule_50moves = 0;
            }
        }
        // add move
        self.state.rule_50moves += 1;

        // if over the limit
        if self.state.rule_50moves >= 100 {
            self.state.mode = GameMode::Finished(None);
            return;
        }
    }

}