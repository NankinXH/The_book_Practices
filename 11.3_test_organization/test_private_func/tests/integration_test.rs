use test_private_func::adder;

#[test]
fn it_adds_2() {
    assert_eq!(4, adder(2, 2));
}
