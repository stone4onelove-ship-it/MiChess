use super::*;


#[derive(Clone)]
pub struct GameLog {
    pub state: GameState,
    pub played: LastPlayed,
}


impl Game {
    pub fn save(&self) -> GameLog {
        GameLog {
            state: self.state.clone(),
            played: self.played.clone(),
        }
    }


    pub fn undo(&mut self) -> bool {
        // check if history is not empty and pop last
        let Some(log) = self.history.pop() else {
            return false;
        };

        // undo state
        self.state = log.state;
        
        // update cache with current dirty, undo transformer with current played
        self.update();
        self.transformer.undo(&NNUE, &self.state.board, self.cache.king_pos, self.played.mv.unwrap());

        // update dirty and last_mv
        self.played = log.played;

        
        true        
    }
}