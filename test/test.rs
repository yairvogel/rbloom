use rbloom::*;

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

#[test]
#[should_panic]
fn new_with_p_greater_than_1() {
    let _b = new(1000, 1.2);
}

#[test]
#[should_panic]
fn new_with_p_smaller_than_0() {
    let _b = new(1000, -0.1);
}

#[test]
#[should_panic]
fn new_exact_k_greater_than_m() {
    let _b = new_exact(100, 10);
}
