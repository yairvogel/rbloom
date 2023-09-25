# rbloom - Fast Bloom Filter
---

rbloom is a small, simple and fast bloom filter, focused on speed and ease of use.
rbloom focuses on bloom filter data structure and doesn't get in the way of the user.

## Features
- **Type Agnostic**: rbloom doesn't enforce type safety by design. As long as your item is [`core::hash::Hash`](https://doc.rust-lang.org/beta/core/hash/trait.Hash.html), it fits into the bloom filter. To enforce any type limitiations, you can always use a thin wrapper around rbloom.
- **Percision Estimation**: a bloom filter guarantees no FNs (returning false when an item is not in the bloom filter) but it might have FPs (returning true even though an item is not in the bloom filter). rbloom lets you tweak that FP rate.
- **Low Memory Footprint**: rbloom uses bitvec as its underlying data manager, which uses compact bit arrangement and access.
 
