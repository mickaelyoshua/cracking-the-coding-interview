pub struct BitVector {
    pub data: Vec<u64>,
}

impl BitVector {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; (size + 63) >> 6],
        }
    }

    pub fn locate(bit: usize) -> (usize, usize) {
        // let block = bit / 64;
        // let position = bit % 64;

        // More efficient, less CPU cicles
        let block = bit >> 6; // equivalente to divide 2^N (2^6=64)
        let position = bit & 63; // extract the rest of the division by 2^N
        (block, position)
    }

    pub fn set(&mut self, bit: usize) {
        let (block, pos) = Self::locate(bit);
        self.data[block] |= 1 << pos;
    }

    pub fn check(&self, bit: usize) -> bool {
        let (block, pos) = Self::locate(bit);
        self.data[block] & (1 << pos) != 0
    }

    pub fn clear(&mut self, bit: usize) {
        let (block, pos) = Self::locate(bit);
        self.data[block] &= !(1 << pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(BitVector::new(30).data.len(), 1);
        assert_eq!(BitVector::new(64).data.len(), 1);
        assert_eq!(BitVector::new(65).data.len(), 2);
        assert_eq!(BitVector::new(128).data.len(), 2);
        assert_eq!(BitVector::new(129).data.len(), 3);
        assert_eq!(BitVector::new(192).data.len(), 3);
    }

    #[test]
    fn test_locate() {
        assert_eq!(BitVector::locate(0), (0, 0));
        assert_eq!(BitVector::locate(63), (0, 63));
        assert_eq!(BitVector::locate(64), (1, 0));
        assert_eq!(BitVector::locate(130), (2, 2));
    }

    #[test]
    fn test_set_and_check() {
        let mut bv = BitVector::new(130);
        assert!(!bv.check(5));

        bv.set(5);
        assert!(bv.check(5));

        bv.set(64);
        assert!(bv.check(64));
        assert!(!bv.check(63));

        bv.set(129);
        assert!(bv.check(129));
    }

    #[test]
    fn test_clear() {
        let mut bv = BitVector::new(100);
        bv.set(50);
        assert!(bv.check(50));

        bv.clear(50);
        assert!(!bv.check(50));
    }
}
