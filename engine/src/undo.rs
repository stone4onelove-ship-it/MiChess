use super::*;


#[derive(Clone)]
pub struct GameLog {
    pub board: Board,
    pub en_passant: Option<u8>,
    pub castle: [[bool;2];2],
    pub player: Color,
    pub mode: GameMode,
    pub rule_50moves: u8,
    pub dirty: BitBoard,
    pub played: Option<PlayedMove>,
}


impl Game {
    pub fn save(&self) -> GameLog {
        GameLog {
            board: self.board.clone(),
            en_passant: self.en_passant,
            castle: self.castle,
            player: self.player,
            mode: self.mode,
            rule_50moves: self.rule_50moves,
            dirty: self.dirty,
            played: self.played,
        }
    }

    pub fn load(&mut self, log: GameLog) -> () {
        self.board = log.board;
        self.en_passant = log.en_passant;
        self.castle = log.castle;
        self.player = log.player;
        self.mode = log.mode;
        self.rule_50moves = log.rule_50moves;
        self.dirty = log.dirty;
        self.played = log.played;

    }

    pub fn undo(&mut self) -> bool {
        // check if the is hostory
        let Some(log) = self.history.pop() else {
            return false;
        };

        // save current values
        let current_dirty = self.dirty;
        let current_played = self.played.unwrap();

        // undo the last move
        self.load(log);
        self.update(current_dirty);

        // undo transformer
        self.transformer.undo(&NNUE, &self.board, self.king_pos, current_played);

        true        
    }
}