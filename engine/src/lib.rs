mod play;
mod update;
mod autoplay;
mod types;
pub use types::*;
mod with_std;
mod undo;
pub use undo::GameLog;
mod nnue;
pub use nnue::{Nnue, Transformer};
mod nnue_file;
pub use nnue_file::NNUE;



#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub played: LastPlayed,
    pub cache: GameCache,
    pub history: Vec<GameLog>,
    pub transformer: Transformer,
}



#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub en_passant: Option<u8>,
    pub castle: [[bool; 2]; 2],
    pub rule_50moves: u8,
    pub player: Color,
    pub mode: GameMode,
}


#[derive(Clone)]
pub struct GameCache {
    // update with self.update
    pub legal: BitGrid,
    pub cover: BitGrid,
    pub cover_comb: [BitBoard;2],
    pub legal_moves: Vec<Move>,
    pub king_pos: [Pos; 2],
    pub check: bool,
}


#[derive(Clone)]
pub struct LastPlayed {
    // updated with self.play
    pub mv: Option<PlayedMove>,
    pub dirty: BitBoard,
}



impl Game {
    pub fn new(board: Board) -> Self {
        let transformer = Transformer::new(&NNUE, &board, board.king_pos());

        let mut game = Game {
            state: GameState { 
                board,
                en_passant: None,
                castle: [[true,true],[true,true]],
                rule_50moves: 0,
                player: Color::White,
                mode: GameMode::Active,
            },
            played: LastPlayed {
                mv: None, 
                dirty: BitBoard(u64::MAX),
            },
            cache: GameCache {
                check: false,
                king_pos: [64;2],
                cover: BitGrid::new(),
                legal: BitGrid::new(),
                cover_comb: [BitBoard::new(), BitBoard::new()],
                legal_moves: Vec::new(),
            },
            history: Vec::new(),
            transformer,
        };
        game.update();
        game
    }
}


// for testing
pub fn timed<F, T>(f: F) -> T 
where F: FnOnce() -> T {
    let start = std::time::Instant::now();
    let result = f();
    println!("took: {:?}", start.elapsed());
    result
}
