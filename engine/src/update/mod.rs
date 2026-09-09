use super::*;

mod cover_comb;
mod legal_moves;
mod pieces;
mod constants;
use constants::*;

// Analyses the board, saves all the game.legal moves and covers for exact board position.
// Changes: game.game.legal , game.w_cover , game.b_cover

impl Game {
    pub fn update(&mut self) -> () {
        // get pieces location BitBoards
        let pieces = board_to_bitboards(&self.state.board);

        // for every dirty piece
        for pos in self.played.dirty.clone().iter_pos() {

            // cleaning
            self.cache.cover[pos] = BitBoard::new();
            self.cache.legal[pos] = BitBoard::new();

            // matching piece
            if let Some(piece) = self.state.board[pos] {
                match piece.role {
                    Role::Pawn => {
                        self.update_pawn(piece, pos, pieces[piece.color.opp() as usize]);
                    }
                    Role::Knight => {
                        self.update_knight(pos, pieces[piece.color as usize]);
                    }
                    Role::Bishop => {
                        self.update_bishop(pos, pieces , piece.color);
                    }
                    Role::Rook => {
                        self.update_rook(pos, pieces , piece.color);
                    }
                    Role::Queen => {
                        self.update_bishop(pos, pieces , piece.color);
                        self.update_rook(pos, pieces , piece.color);
                    }
                    Role::King => {
                        self.update_king_cover(pos);
                        self.cache.king_pos[piece.color as usize] = pos;
                    }
                }
            }
        }

        // update cover comb 
        self.update_cover_comb();

        // dirty king legal updated last and fill king pos 
        self.update_king_legal(self.cache.king_pos[0], pieces[0]);
        self.update_king_legal(self.cache.king_pos[1], pieces[1]);

        // save legal moves
        self.update_legal_moves();
        // check if there is a check
        self.cache.check = self.cache.cover_comb[self.state.player.opp() as usize].get(self.cache.king_pos[self.state.player as usize]);
    }


}




fn board_to_bitboards(board: &Board) -> [BitBoard;2] {
    let mut white = BitBoard::new();
    let mut black = BitBoard::new();
    
    for (i, square) in board.0.iter().enumerate() {
        if let Some(piece) = square {
            match piece.color {
                Color::White => white.0 |= 1u64 << i,
                Color::Black => black.0 |= 1u64 << i,
            }
        }
    }
    [white, black]
}

