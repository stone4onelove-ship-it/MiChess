use super::*;

const MATE_VALUE: [i32; 2] =  [1_000_000, -1_000_000];


pub fn analyze(config: &Config, game: &mut Game, depth: i8, iterated: &mut i32) -> i32 {

    let mut moves: Vec<(Move, i32)> = Vec::new(); 

    for mv in game.cache.legal_moves.clone() {
        if game.play(mv).is_ok() {
            *iterated += 1;

            // if game is finished
            if let Some(value) = status(&game.state.mode) {
                moves.push((mv, value));
                game.undo();
                continue;
            }

            let mut value = game.eval();

            // PLAYER
            if game.state.player == config.init_player {

                let deeper = depth <= config.max_depth;
                // go deeper if needed
                if deeper {
                    value = analyze(config, game, depth + 1, iterated);
                } 

            // OPONENT
            } else {

                let deeper = depth <= config.max_depth;
                // go deeper if needed
                if deeper {
                    value = analyze(config, game, depth + 1, iterated);
                }
                
            }

            // save move info
            moves.push((mv, value));
            game.undo();
        }
    }

    // choose worst player outcome
    let chosen_move = if config.init_player == Color::White {
        moves.iter().max_by_key(|x| x.1).unwrap().1
    } else {
        moves.iter().min_by_key(|x| x.1).unwrap().1
    };

    
    return chosen_move
}




// check if game is runnig or it is finished
fn status(mode: &GameMode) -> Option<i32> {
    match mode {
        &GameMode::Active => None,
        &GameMode::Finished(None) => Some(0),
        &GameMode::Finished(Some(color)) => Some(MATE_VALUE[color as usize]),
    }
}