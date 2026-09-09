use super::*;



mod analyze;
use analyze::{analyze};
mod eval;
mod choose_move;
use choose_move::choose_move;



struct Config {
    init_player: Color,
    max_depth: i8
}


impl Game {
    pub fn autoplay(&mut self) -> () {
        // clone for safety
        let game = &mut self.clone(); 

        let mut moves_iterated: Vec<i32> = Vec::new();

        let config = Config { init_player: game.state.player, max_depth: 2 };

        let mut moves: Vec<(Move, i32)> = Vec::new(); 

        // iterating through all legal moves
        for &mv in &self.cache.legal_moves {
            if game.play(mv).is_ok() {
                let mut iterated = 0;
                
                let value = analyze(&config, game, 1, &mut iterated);

                moves_iterated.push(iterated);

                moves.push((mv, value));
                game.undo();
            } 
        }

        // make move
        let mv: Move = choose_move(&mut moves);
        self.play(mv).unwrap();

        // console ouput
        println!("\nIteratrions done : {:?}", moves_iterated);
        println!("\n---Bot makes move!---\nchosen move: {mv:?}\n");
    }
}




