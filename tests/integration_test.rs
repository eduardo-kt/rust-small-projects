use rust_small_projs::simple_tests;

#[test]
fn try_add() {
    assert_eq!(5, simple_tests::simple_sum(2,3));
}