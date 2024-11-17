use enum_to_vec::ToVec;

#[derive(ToVec)]
#[snake_case]
enum Test {
    Test,
    TestA,
    TestB,
}
#[test]
fn test_vec() {
    assert_eq!(vec!["test", "test_a", "test_b"], Test::to_vec());
}
