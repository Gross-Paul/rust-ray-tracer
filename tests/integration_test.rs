
// New mod
mod sub;

/// Integration test
#[test]
fn check_good() {
    assert_eq!(2.0, sub::add(1.0, 1.0));
}