
fn main() {
    let mut bloom_filter = rbloom::new(100, 0.01);
    
    assert_eq!(bloom_filter.test(&"hello world"), false);
    
    bloom_filter.add(&"hello world");

    assert_eq!(bloom_filter.test(&"hello world"), true);

    println!("success");
}