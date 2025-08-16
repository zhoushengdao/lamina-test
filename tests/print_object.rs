use lamina_test::s;
use rstest::rstest;

#[rstest]
#[case("pi()", "π")]
#[case("e()", "e")]
#[case("true", "true")]
#[case("false", "false")]
#[case("null", "null")]
#[case(r#""str""#, "str")]
#[case(r#""excape\"str""#, r#"excape"str"#)]
#[case(r#""comma,str""#, r#"comma,str"#)]
#[case(r#""中文支持""#, "中文支持")]
fn builtin_object(#[case] input: String, #[case] expected: String) {
    assert_eq!(expected, s(input.clone()), "简单表达式“{}”", input);
}
