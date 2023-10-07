use bitvec::prelude::*;
use fxhash::FxHasher;
use core::hash::{Hash, Hasher};

pub struct BloomFilter {
    bit_vec: BitVec,
    k: usize
}

impl BloomFilter {
    /// Adds an item to the bloom filter.
    /// ```
    /// # use rbloom::*;
    /// # let mut bloom_filter = new_exact(3, 10);
    /// bloom_filter.add(&"hello world");
    /// assert_eq!(true, bloom_filter.test(&"hello world"));
    /// ```
    pub fn add<T>(&mut self, item: &T)
        where T: Hash {
        for hash in get_hashes(item, self.k) {
            let idx = hash as usize % self.bit_vec.len();
            *self.bit_vec.get_mut(idx).unwrap() = true;
        }
    }

    /// Tests for existance of an item in the bloom filter.
    /// ```
    /// # use rbloom::*;
    /// # let mut bloom_filter = new_exact(3, 10);
    /// let found = bloom_filter.test(&"hello world");
    /// assert_eq!(false, found);
    /// ```
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

/// creates a new bloom filter struct, for approximately `n` items with false positive rate of `p`
/// Example:
/// ```
/// let bloom_filter = rbloom::new(1000, 0.001);
/// ```
/// this code creates a new bloom filter that for approximately 1000 items will have a false
/// positive rate of 0.1%.
/// ```
///
pub fn new(n: usize, p: f64) -> BloomFilter {
    let log2 = 2.0f64.ln();
    let m = (((n as f64) * (p as f64).ln() * -1.0f64) / (log2 * log2)).ceil() as usize;
    assert!(m > 0, "m = {}", m);
    let k = ((p as f64).log2() * -1.0f64 / log2).ceil() as usize;
    new_exact(k, m)
}

/// creates a bloom filter struct of length `m` using `k` hash functions per operation
/// Example:
/// ```
/// let bloom_filter = rbloom::new_exact(2, 100);
/// ```
pub fn new_exact(k: usize, m: usize) -> BloomFilter {
    assert!(k < m, "k cannot be larger than m");
    BloomFilter {
        bit_vec: bitvec![0; m],
        k
    }
}

