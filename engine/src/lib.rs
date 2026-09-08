mod play;
mod update;
mod autoplay;
mod types;
pub use types::*;
mod state;
pub use state::State;
mod undo;
pub use undo::GameLog;
mod nnue;
pub use nnue::{Nnue, Transformer};


// for testing 
pub static NNUE: Nnue = Nnue {
    feature_weights: [[0.2; 128]; 49163],
    feature_bias: [0.3; 128],

    hidden0_weights: [0.0; 32 * 256],
    hidden0_bias: [0.0; 32],

    hidden1_weights: [0.0; 32 * 32],
    hidden1_bias: [0.0; 32],

    output_weights: [0.0; 32],
    output_bias: 0.0,
};



#[derive(Clone)]
pub struct Game {

    // game info
    pub(crate) board: Board,
    pub(crate) en_passant: Option<u8>,
    pub(crate) castle: [[bool; 2]; 2],
    pub(crate) rule_50moves: u8,

    // check
    pub(crate) check: bool,

    // players
    pub(crate) player: Color,
    
    // mode
    pub(crate) mode: GameMode,
    
    // king pos
    pub(crate) king_pos: [Pos; 2],

    // moves
    pub(crate) legal: BitGrid,
    pub(crate) cover: BitGrid,
    pub(crate) cover_comb: [BitBoard;2],
    pub(crate) legal_moves: Vec<Move>,

    // for dynamic update
    pub(crate) played: Option<PlayedMove>,
    pub(crate) dirty: BitBoard,

    // history
    pub(crate) history: Vec<GameLog>,

    // for nnue
    pub(crate) transformer: Transformer,

}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        let king_pos = board.king_pos();
        let transformer = Transformer::new(&NNUE, &board, king_pos);

        let mut game = Game {
            // main game info
            board,
            en_passant: None,
            castle: [[true,true],[true,true]],
            rule_50moves: 0,
            // player
            player: Color::White,
            // mode
            mode: GameMode::Active,


            // check
            check: false,

            
            //(updated by self.update)
            // king pos 
            king_pos,
            // moves
            cover: BitGrid::new(),
            legal: BitGrid::new(),
            cover_comb: [BitBoard::new(), BitBoard::new()],
            legal_moves: Vec::new(),

            // dynamic update
            played: None, 
            dirty: BitBoard::new(),

            // game history
            history: Vec::new(),

            transformer,

        };
        game.update(BitBoard(u64::MAX));
        game
    }
}



pub fn timed<F, T>(f: F) -> T 
where F: FnOnce() -> T {
    let start = std::time::Instant::now();
    let result = f();
    println!("took: {:?}", start.elapsed());
    result
}
