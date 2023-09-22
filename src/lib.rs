use bitvec::prelude::*;
use fxhash::FxHasher;
use core::hash::{Hash, Hasher};

pub struct BloomFilter {
    bit_vec: BitVec,
    k: usize
}

impl BloomFilter {
    pub fn add<T>(&mut self, item: &T)
        where T: Hash {
        for hash in get_hashes(item, self.k) {
            let idx = hash as usize % self.bit_vec.len();
            *self.bit_vec.get_mut(idx).unwrap() = true;
        }
    }

    pub fn test<T>(&self, item: &T) -> bool
        where T: Hash {
            get_hashes(item, self.k)
                .all(|hash| *self.bit_vec.get(hash as usize % self.bit_vec.len()).unwrap())
        }
}

fn get_hashes<'a, T>(item: &'a T, k: usize) -> impl Iterator<Item = u32> + 'a where T : Hash {
    (0..k).map(move |seed| hash_with_seed(item, seed))
}

fn hash_with_seed<T>(item: &T, seed: usize) -> u32 where T: Hash {
    let mut hasher = FxHasher::default();
    seed.hash(&mut hasher);
    item.hash(&mut hasher);
    hasher.finish() as u32
}

pub fn new(n: usize, p: f64) -> BloomFilter {
    let log2 = 2.0f64.ln();
    let m = (((n as f64) * (p as f64).ln() * -1.0f64) / (log2 * log2)).ceil() as usize;
    assert!(m > 0, "m = {}", m);
    let k = ((p as f64).log2() * -1.0f64 / log2).ceil() as usize;
    new_exact(k, m)
}

pub fn new_exact(k: usize, m:usize) -> BloomFilter {
    assert!(k < m, "k cannot be larger than m");
    BloomFilter {
        bit_vec: bitvec![0; m],
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
    #[should_panic(expected = "k cannot be larger than m")]
    fn test_panic_when_k_is_larger_than_m() {
        new_exact(100, 10);
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

    #[test]
    fn test_with_different_types() {
        let mut b = new(1000, 0.01);
        assert_eq!(b.test(&false), false);
        b.add(&false);
        assert_eq!(b.test(&false), true);

        assert_eq!(b.test(&150), false);
        b.add(&150);
        assert_eq!(b.test(&150), true);
    }

    #[test]
    fn test_deterministic() {
        let mut b1 = new_exact(3, 100);
        let mut b2 = new_exact(3, 100);
        b1.add(&"hello world");
        b2.add(&"hello world");
        assert_eq!(b1.bit_vec, b2.bit_vec);
    }

    #[test]
    fn test_different_seeds() {
        let mut b = new_exact(3, 100);
        b.add(&"hello world");
        assert_eq!(b.bit_vec.as_bitslice().count_ones(), 3);
    }
}