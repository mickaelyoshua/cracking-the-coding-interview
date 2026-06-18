#[derive(Default)]
pub struct BitVector {
    pub data: u32,
}

impl BitVector {
    pub fn new() -> Self {
        Self { data: 0 }
    }

    pub fn set(&mut self, bit: usize) {
        self.data |= 1 << bit;
    }

    pub fn check(&self, bit: usize) -> bool {
        self.data & (1 << bit) != 0
    }
}
