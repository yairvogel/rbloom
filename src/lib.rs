use std::io::Cursor;
use bitvec::prelude::*;
use murmur3::murmur3_32;

pub struct BloomFilter {
    bit_vec: BitVec,
    k: usize
}

impl BloomFilter {
    pub fn add<T>(&mut self, item: &T)
        where T: AsRef<[u8]> {
        for hash in self.get_hashes(&item) {
            let idx = hash as usize % self.bit_vec.len();
            *self.bit_vec.get_mut(idx).unwrap() = true;
        }
    }

    pub fn test<T>(&self, item: &T) -> bool
        where T: AsRef<[u8]> {
            self.get_hashes(&item)
                .all(|hash| *self.bit_vec.get(hash as usize % self.bit_vec.len()).unwrap())
        }

    fn get_hashes<'a, T>(&self, item: &'a T) -> impl Iterator<Item = u32> + 'a where T : AsRef<[u8]> {
        (0..self.k).map(move |seed| murmur3_32(&mut Cursor::new(item), seed as u32).unwrap())
    }
}

pub fn new(n: usize, p: f64) -> BloomFilter {
    let log2 = 2.0f64.ln();
    let m = (((n as f64) * (p as f64).ln() * -1.0f64) / (log2 * log2)).ceil() as usize;
    assert!(m > 0, "m = {}", m);
    let k = ((p as f64).log2() * -1.0f64 / log2).ceil() as usize;
    new_exact(k, m)
}

pub fn new_exact(k: usize, m:usize) -> BloomFilter {
    let mut bit_vec = BitVec::from_element(0usize);
    bit_vec.resize(m, false);
    BloomFilter {
        bit_vec,
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_correctly() {
        let b = new_exact(7, 100);
        assert!(b.k == 7);
        assert!(b.bit_vec.len() == 100);
    }

    #[test]
    fn test_calculate_estimates_positive_result() {
        let b = new(100, 0.01);
        assert!(b.bit_vec.len() > 0, "m = {}", b.bit_vec.len());
        assert!(b.k > 0, "k = {}", b.k);
    }

    #[test]
    fn test_add_and_test() {
        let mut b = new(100, 0.01);
        assert_eq!(b.test(&"hello world"), false);
        b.add(&"hello world");
        assert_eq!(b.test(&"hello world"), true);
    }

    #[test]
    fn test_test_for_non_existing() {
        let mut b = new(100, 0.01);
        b.add(&"hello world");
        assert_eq!(b.test(&"hello"), false);
    }
}