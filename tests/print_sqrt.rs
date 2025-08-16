use lamina_test::s;
use rstest::rstest;

#[rstest]
#[case("pi()", "√π")]
#[case("e()", "√e")]
#[case("true", "Error: sqrt() requires numeric argument")]
#[case("false", "Error: sqrt() requires numeric argument")]
#[case("null", "Error: sqrt() requires numeric argument")]
#[case(r#""str""#, "Error: sqrt() requires numeric argument")]
fn builtin_object(#[case] input: String, #[case] expected: String) {
    assert_eq!(
        expected,
        s(format!("sqrt({})", input.clone())),
        "简单表达式“sqrt({})”",
        input
    );
}

#[rstest]
#[case("0", "0")]
#[case("-0", "0")]
#[case("1", "1")]
#[case("-1", "Error: sqrt() of negative number")]
fn normal_number(#[case] input: String, #[case] expected: String) {
    assert_eq!(
        expected,
        s(format!("sqrt({})", input.clone())),
        "简单表达式“sqrt({})”",
        input
    );
}

#[rstest]
#[case("10", "√10")]
#[case("100", "10")]
#[case("1000", "10√10")]
#[case("10000", "100")]
#[case("100000", "100√10")]
#[case("1000000", "1000")]
#[case("10000000", "1000√10")]
#[case("100000000", "10000")]
#[case("1000000000", "10000√10")]
#[case("10000000000", "100000")]
#[case("100000000000", "100000√10")]
#[case("1000000000000", "1000000")]
#[case("1000000000000000", "10000000√10")]
#[case("1000000000000000000", "1000000000")]
#[case("1000000000000000000000", "10000000000√10")]
#[case("1000000000000000000000000", "1000000000000")]
#[case("1000000000000000000000000000", "10000000000000√10")]
#[case("1000000000000000000000000000000", "1000000000000000")]
fn big_number(#[case] input: String, #[case] expected: String) {
    assert_eq!(
        expected,
        s(format!("sqrt({})", input.clone())),
        "简单表达式“sqrt({})”",
        input
    );
}

#[rstest]
#[case("0.1", "√10/10")] // TODO
#[case("0.01", "1/10")] // TODO
#[case("0.001", "√10/100")] // TODO
#[case("0.0001", "1/100")] // TODO
#[case("0.00001", "√10/1000")] // TODO
#[case("0.000001", "1/1000")] // TODO
#[case("0.0000001", "√10/10000")] // TODO
#[case("0.00000001", "1/10000")] // TODO
#[case("0.000000001", "√10/100000")] // TODO
#[case("0.0000000001", "1/100000")] // TODO
#[case("0.00000000001", "√10/1000000")] // TODO
#[case("0.000000000001", "1/1000000")] // TODO
#[case("0.000000000000001", "√10/100000000")] // TODO
#[case("0.000000000000000001", "1/1000000000")] // TODO
#[case("0.000000000000000000001", "√10/100000000000")] // TODO
#[case("0.000000000000000000000001", "1/1000000000000")] // TODO
#[case("0.000000000000000000000000001", "√10/100000000000000")] // TODO
#[case("0.000000000000000000000000000001", "1/1000000000000000")] // TODO
fn small_number(#[case] input: String, #[case] expected: String) {
    assert_eq!(
        expected,
        s(format!("sqrt({})", input.clone())),
        "简单表达式“sqrt({})”",
        input
    );
}

#[rstest] // ! 循环引用警告
#[case("1/pi()", "√π/π")] // TODO
#[case("1/e()", "√e/e")] // TODO
#[case("pi()/1", "√π")]
#[case("pi()/2", "√π/2")] // TODO
#[case("pi()/3", "√π/3")] // TODO
#[case("e()/1", "√e")]
#[case("e()/2", "√e/2")] // TODO
#[case("e()/3", "√e/3")] // TODO
fn frac_builtin_object(#[case] input: String, #[case] expected: String) {
    assert_eq!(
        expected,
        s(format!("sqrt({})", input.clone())),
        "简单表达式“sqrt({})”",
        input
    );
}

#[rstest]
#[case("2", "√2")]
#[case("4", "2")]
#[case("8", "2√2")]
fn special(#[case] input: String, #[case] expected: String) {
    assert_eq!(
        expected,
        s(format!("sqrt({})", input.clone())),
        "简单表达式“sqrt({})”",
        input
    );
}
