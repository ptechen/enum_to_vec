use enum_to_vec::ToVec;

#[derive(ToVec)]
#[ignore_segments]
enum Test {
    Test,
    TestA,
    TestB,
    TEsst(String, String),
    TE(Data)
}
pub struct Data {}
#[test]
fn test_vec() {
    println!("{:?}", Test::to_vec());
}
