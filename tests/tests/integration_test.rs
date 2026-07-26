use tests::add;

mod common;

#[test]
fn it_adds() {
    common::setup();
    
    let result = add(2, 2);
    assert_eq!(result, 4);
}