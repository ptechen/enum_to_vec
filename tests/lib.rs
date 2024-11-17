use enum_to_vec::ToVec;

#[derive(ToVec)]
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
