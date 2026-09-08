use super::*;

mod constants;
use constants::*;




impl Game {
    pub fn eval(&self) -> i32 {
        let mut value = 0;

        for pos in 0..64 {
            if let Some(piece) = self.board[pos] {
                match piece.color {
                    Color::White => {
                        value += eval_material(piece.role);
                        value += eval_value_grid(pos, piece);
                    }
                    Color::Black => {
                        value -= eval_material(piece.role);
                        value -= eval_value_grid(pos, piece);
                    }
                }
            }
        }

        value += self.eval_king_safety();
        value += self.eval_mobility();


        value
    }



    fn eval_king_safety(&self) -> i32 {
        0
    }



    fn eval_mobility(&self) -> i32 {
        0
    }


}


// use piece grids to get piece pos worth
fn eval_value_grid(pos: Pos, piece: Piece) -> i32 {
    match piece.role {
        Role::Pawn   =>  PAWN_VALUE_GRID[piece.color as usize][pos as usize],
        Role::Knight =>  KNIGHT_VALUE_GRID[pos as usize],
        Role::Bishop =>  BISHOP_VALUE_GRID[pos as usize],
        Role::Rook   =>  ROOK_VALUE_GRID[pos as usize],
        Role::Queen  =>  QUEEN_VALUE_GRID[pos as usize],
        Role::King   =>  0,
    }
}



// get how much a piece is worth
fn eval_material(role : Role) -> i32 {
    match role {
        Role::Pawn   =>  100,
        Role::Knight =>  300,
        Role::Bishop =>  500,
        Role::Rook   =>  300,
        Role::Queen  =>  900,
        Role::King   => 1500,
    }
}




