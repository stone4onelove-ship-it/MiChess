use super::*;

#[derive(Copy, Clone, PartialEq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const fn new() -> Self {
        BitBoard(0)
    }

    pub fn set(&mut self, pos: Pos) -> () {
        self.0 |= 1u64 << pos;
    }

    pub fn set_all(&mut self) -> () {
        self.0 = u64::MAX;
    }

    pub fn clear(&mut self, pos: Pos) -> () {
        self.0 &= !(1u64 << pos);
    }

    pub fn get(&self, pos: Pos) -> bool {
        self.0 & (1u64 << pos) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> {
        let mut bits = self.0;
        core::iter::from_fn(move || {
            if bits == 0 {
                None
            } else {
                let pos = bits.trailing_zeros() as u8;
                bits &= bits - 1;
                Some(pos)
            }
        })
    }
}
