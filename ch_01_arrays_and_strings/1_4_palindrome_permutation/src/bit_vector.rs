#[derive(Default)]
pub struct BitVector {
    pub data: u32,
}

impl BitVector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self, bit: usize) {
        let mask = 1 << bit;
        self.data ^= mask;
    }

    pub fn check(&self, bit: usize) -> bool {
        let mask = 1 << bit;
        self.data & mask != 0
    }
}
