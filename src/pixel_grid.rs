use crate::{BitVec, Packet};

#[derive(Debug)]
pub struct PixelGrid {
    pub width: usize,
    pub height: usize,
    bit_vec: BitVec,
}

impl PixelGrid {
    pub fn new(width: usize, height: usize) -> PixelGrid {
        assert_eq!(width % 8, 0);
        assert_eq!(height % 8, 0);
        Self {
            width,
            height,
            bit_vec: BitVec::new(width * height),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        self.bit_vec.set(x + y * self.width, value)
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.bit_vec.get(x + y * self.width)
    }
}

impl From<PixelGrid> for Packet {
    fn from(value: PixelGrid) -> Self {
        value.bit_vec.into()
    }
}
