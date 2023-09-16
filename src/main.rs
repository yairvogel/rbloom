use rbloom::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::<7>::new();
    
    assert_eq!(bloom_filter.test(&"hello world"), false);
    
    bloom_filter.add(&"hello world");

    assert_eq!(bloom_filter.test(&"hello world"), true);

    let sum: usize = bloom_filter.0.iter().map(|x| *x as usize).sum();
    assert_eq!(sum, 7);

    assert_eq!(bloom_filter.test(&"hello"), false);
    bloom_filter.add(&"hello");
    assert_eq!(bloom_filter.test(&"hello"), true);

    println!("success");
}