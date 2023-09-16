use std::io::Cursor;
use bitvec::prelude::*;
use murmur3::murmur3_32;

pub struct BloomFilter<const K: usize>(pub BitArr!(for 1 << 3, in usize));

impl<const K: usize> BloomFilter<K> {
    pub fn new() -> Self {
        BloomFilter(bitarr![0; 1 << 3])
    }

    pub fn add<T>(&mut self, item: &T)
        where T: AsRef<[u8]> {
        for hash in self.get_hashes(&item) {
            let idx = hash as usize % self.0.len();
            *self.0.get_mut(idx).unwrap() = true;
        }
    }

    pub fn test<T>(&self, item: &T) -> bool
        where T: AsRef<[u8]> {
            self.get_hashes(&item)
                .iter()
                .all(|hash| *self.0.get(*hash as usize % self.0.len()).unwrap())
        }

    fn get_hashes<T>(&self, item: &T) -> [u32; K] where T : AsRef<[u8]> {
        let mut hashes = [0u32; K];
        for seed in 0..K {
            hashes[seed] = murmur3_32(&mut Cursor::new(item), seed as u32).unwrap();
        }
        hashes
    }
}