use adder;

mod common;

#[test]
fn it_adds_two_again() {
    common::setup();
    assert_eq!(5, adder::add_two(3))
}
