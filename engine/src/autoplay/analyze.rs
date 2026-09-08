use super::*;

const PINF: i32 =  10_000_000;
const NINF: i32 = -10_000_000;



pub fn analyze(config: &Config, game: &mut Game, depth: i8, iterated: &mut i32) -> i32 {

    let mut moves: Vec<( Move , i32 )> = Vec::new(); 


    for mv in game.legal_moves.clone() {

        if game.play(mv) {
            *iterated += 1;

            let status = status_check(&game.mode, config.init_player);
            if status != 0 {
                moves.push((mv, status));
                game.undo();
                continue;
            }

            let mut value = game.eval();

            // PLAYER
            if game.player == config.init_player {

                let deeper = true;
                // go deeper if needed
                if deeper {
                    value = analyze(config, game, depth + 1, iterated);
                } 

            // OPONENT
            } else {

                let deeper = true;
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
    let chosen_move: i32;
    if game.player == config.init_player {
        chosen_move = moves.iter().max_by_key(|x| x.1).unwrap().1;
    } else {
        chosen_move = moves.iter().min_by_key(|x| x.1).unwrap().1;
    }

    
    return chosen_move
}




// check if game is runnig or it is finished
fn status_check(mode: &GameMode, init_player: Color ) -> i32 {
    match mode {
        &GameMode::Active | &GameMode::Finished(None) => 0,
        &GameMode::Finished(Some(value)) => { if value == init_player { PINF } else { NINF }}
    }
}