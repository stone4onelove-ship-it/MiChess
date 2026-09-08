use super::*;

mod get_dirty;
mod checks;


impl Game {
    // makes a move, returns bool
    pub fn play(&mut self, mv: Move) -> bool {
        let (start_pos, end_pos) = mv;
        // checks if the move is legal
        if self.mode != GameMode::Active {
            println!("Move failed : wrong mode");
            return false;
        }
        if !self.board[start_pos].is_some_and(|p| p.color == self.player) {
            println!("Move failed : wrong start pos");
            return false;
        }
        if !self.legal[start_pos].get(end_pos) {
            println!("Move failed : wrong end pos");
            return false;
        };
        
        
        // saving history
        self.history.push(self.save());

        // MAKING THE MOVE

        // en passant
        let en_passant_copy = self.en_passant;
        self.en_passant = None;

        // saving played move
        let mut played = PlayedMove {mv, tp: MoveType::Basic, captured: self.board[end_pos]};

        let piece = self.board[start_pos].unwrap();
        match piece.role {
            Role::Pawn => {
                // play en passant
                if Some(end_pos.col()) == en_passant_copy {
                    let op_pawn_pos = start_pos.row()*8 + end_pos.col();
                    if self.board[op_pawn_pos] == Some(Piece{color: self.player.opp(), role: Role::Pawn}) {
                        // save move data
                        played.captured = self.board[op_pawn_pos];
                        played.tp = MoveType::EnPassant;
                        // capture opp pawn
                        self.board[op_pawn_pos] = None;
                    }
                }

                // set en passant
                if (start_pos.row() as i8 - end_pos.row() as i8).abs() == 2 {
                    self.en_passant = Some(start_pos.col());
                }
            },
            Role::Rook => {
                // cancel castle
                match start_pos.col() {
                    0 => self.castle[piece.color as usize][0] = false,
                    7 => self.castle[piece.color as usize][1] = false,
                    _     => {},
                }
            },
            Role::King => {
                // make castle
                let row = match piece.color {Color::White => 7, Color::Black => 0};
                if start_pos.row() == row && end_pos.row() == row && start_pos.col() == 4 {
                    match end_pos.col() {
                        2 => { 
                            self.board[row*8 + 3] = Some(Piece{color: self.player, role: Role::Rook}); 
                            self.board[row*8 + 0] = None;
                            // save move data
                            played.tp = MoveType::Castle((row*8 + 0, row*8 + 3));
                        },
                        6 => { 
                            self.board[row*8 + 5] = Some(Piece{color: self.player, role: Role::Rook}); 
                            self.board[row*8 + 7] = None;
                            // save move data
                            played.tp = MoveType::Castle((row*8 + 7, row*8 + 5));
                        },
                        _  => {},
                    }
                }
                // cancel castle
                self.castle[piece.color as usize] = [false, false];
            },
            _ => {}
        }


        // possible promotion or basic move
        if piece.role == Role::Pawn && (end_pos < 8 || end_pos >= 56) {
            self.board[end_pos] = Some(Piece{color: self.player, role: Role::Queen});
            self.board[start_pos] = None;
            played.tp = MoveType::Promotion;
        } else {
            self.board[end_pos] = Some(piece);
            self.board[start_pos] = None;
        }

        // saving played move
        self.played = Some(played);

        // change the player
        self.player = self.player.opp();

        // update info
        self.get_dirty();
        self.update(self.dirty);

        // update transformer
        self.transformer.play(&NNUE, &self.board, self.king_pos, played);

        // check if move is legal, if not loads back up 
        let king_pos: Pos = self.king_pos[self.player.opp() as usize];
        if self.cover_comb[self.player as usize].get(king_pos) {
            self.undo();
            println!("Move failed : king in danger");
            return false;
        }
        
        // check for wins and draws
        self.check_check();
        self.win_check();
        self.stalemate_check();
        self.no_material_check();
        self.rule_50_check();
        
        return true;
    }
}


    

