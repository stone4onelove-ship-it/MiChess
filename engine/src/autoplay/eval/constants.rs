pub(super) const PAWN_VALUE_GRID: [[i32; 64];2] = [[0; 64];2];

pub(super) const KNIGHT_VALUE_GRID: [i32; 64] = [
    -35,-25,-15,-15,-15,-15,-25,-35,
    -25,-10,  0,  5,  5,  0,-10,-25,
    -10,  0, 10, 25, 25, 10,  0,-10,
     -5, 15, 35, 50, 50, 35, 15, -5,
     -5, 15, 35, 50, 50, 35, 15, -5,
    -10,  0, 10, 25, 25, 10,  0,-10,
    -25,-10,  0,  5,  5,  0,-10,-25,
    -35,-25,-15,-15,-15,-15,-25,-35,
];

pub(super) const BISHOP_VALUE_GRID: [i32; 64] = [0; 64];

pub(super) const ROOK_VALUE_GRID: [i32; 64] = [0; 64];

pub(super) const QUEEN_VALUE_GRID: [i32; 64] = [0; 64];

pub(super) const KING_MIDGAME_VALUE_GRID: [i32; 64] = [0; 64];

pub(super) const KING_ENDGAME_VALUE_GRID: [i32; 64] = [0; 64];


