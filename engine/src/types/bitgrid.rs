use super::*;


#[derive(Copy, Clone)]
pub struct BitGrid(pub [BitBoard; 64]);

impl BitGrid {
    pub const fn new() -> Self {
        BitGrid([BitBoard::new(); 64])
    }
}

impl core::ops::Index<Pos> for BitGrid {
    type Output = BitBoard;
    fn index(&self, pos: Pos) -> &Self::Output {
        &self.0[pos as usize]
    }
}

impl core::ops::IndexMut<Pos> for BitGrid {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.0[pos as usize]
    }
}


