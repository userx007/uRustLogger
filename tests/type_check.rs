use trybuild::TestCases;

#[test]
fn test_type_safety() {
    let t = TestCases::new();
    t.compile_fail("tests/type_fail/*.rs");
}
