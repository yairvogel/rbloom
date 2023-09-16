use rbloom::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::new();
    
    assert_eq!(bloom_filter.test(&"hello world"), false);
    
    bloom_filter.add(&"hello world");

    assert_eq!(bloom_filter.test(&"hello world"), true);
    assert_eq!(bloom_filter.test(&"hello"), false);
    println!("success");
}