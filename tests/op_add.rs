mod op_add_builtin_object {
    use lamina_test::s;
    use rstest::rstest;

    #[rstest]
    // pi
    #[case("pi()+pi()", "2π")]
    #[case("pi()+e()", "π+e")]
    #[case("pi()+true", "Error: Cannot add π and true")]
    #[case("pi()+false", "Error: Cannot add π and false")]
    #[case("pi()+null", "Error: Cannot add π and null")]
    #[case(r#"pi()+"str""#, r#"Error: Cannot add π and "str""#)] // TODO
    // e
    #[case("e()+pi()", "e+π")]
    #[case("e()+e()", "2e")]
    #[case("e()+true", "Error: Cannot add e and true")]
    #[case("e()+false", "Error: Cannot add e and false")]
    #[case("e()+null", "Error: Cannot add e and null")]
    #[case(r#"e()+"str""#, r#"Error: Cannot add e and "str""#)] // TODO
    // true
    #[case("true+pi()", "Error: Cannot add true and π")]
    #[case("true+e()", "Error: Cannot add true and e")]
    #[case("true+true", "Error: Cannot add true and true")]
    #[case("true+false", "Error: Cannot add true and false")]
    #[case("true+null", "Error: Cannot add true and null")]
    #[case(r#"true+"str""#, r#"Error: Cannot add true and "str""#)]
    // false
    #[case("false+pi()", "Error: Cannot add false and π")]
    #[case("false+e()", "Error: Cannot add false and e")]
    #[case("false+true", "Error: Cannot add false and true")]
    #[case("false+false", "Error: Cannot add false and false")]
    #[case("false+null", "Error: Cannot add false and null")]
    #[case(r#"false+"str""#, r#"Error: Cannot add false and "str""#)]
    // null
    #[case("null+pi()", "Error: Cannot add null and π")]
    #[case("null+e()", "Error: Cannot add null and e")]
    #[case("null+true", "Error: Cannot add null and true")]
    #[case("null+false", "Error: Cannot add null and false")]
    #[case("null+null", "Error: Cannot add null and null")]
    #[case(r#"null+"str""#, r#"Error: Cannot add null and "str""#)]
    // "str"
    #[case(r#""str"+pi()"#, r#"Error: Cannot add "str" and π"#)]
    #[case(r#""str"+e()"#, r#"Error: Cannot add "str" and e"#)]
    #[case(r#""str"+true"#, r#"Error: Cannot add "str" and true"#)]
    #[case(r#""str"+false"#, r#"Error: Cannot add "str" and false"#)]
    #[case(r#""str"+null"#, r#"Error: Cannot add "str" and null"#)]
    #[case(r#""str"+"str""#, "strstr")]
    fn builtin_object(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, s(input.clone()), "简单表达式“{}”", input);
    }

    #[rstest]
    #[case("pi()+0", "π")]
    #[case("pi()+-0", "π")]
    #[case("pi()+1", "π+1")]
    #[case("pi()+-1", "π-1")]
    #[case("e()+0", "e")]
    #[case("e()+-0", "e")]
    #[case("e()+1", "e+1")]
    #[case("e()+-1", "e-1")]
    #[case("true+0", "Error: Cannot add true and 0")]
    #[case("true+-0", "Error: Cannot add true and 0")]
    #[case("true+1", "Error: Cannot add true and 1")]
    #[case("true+-1", "Error: Cannot add true and -1")]
    #[case("false+0", "Error: Cannot add false and 0")]
    #[case("false+-0", "Error: Cannot add false and 0")]
    #[case("false+1", "Error: Cannot add false and 1")]
    #[case("false+-1", "Error: Cannot add false and -1")]
    #[case("null+0", "Error: Cannot add null and 0")]
    #[case("null+-0", "Error: Cannot add null and 0")]
    #[case("null+1", "Error: Cannot add null and 1")]
    #[case("null+-1", "Error: Cannot add null and -1")]
    #[case(r#""str"+0"#, r#"Error: Cannot add "str" and 0"#)]
    #[case(r#""str"+-0"#, r#"Error: Cannot add "str" and 0"#)]
    #[case(r#""str"+1"#, r#"Error: Cannot add "str" and 1"#)]
    #[case(r#""str"+-1"#, r#"Error: Cannot add "str" and -1"#)]
    fn normal_number(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, s(input.clone()), "简单表达式“{}”", input);
    }

    #[rstest]
    #[case("pi()+10", "π+10")]
    #[case("pi()+100", "π+100")]
    #[case("pi()+1000", "π+1000")]
    #[case("pi()+10000", "π+10000")]
    #[case("pi()+100000", "π+100000")]
    #[case("pi()+1000000", "π+1000000")]
    #[case("pi()+10000000", "π+10000000")]
    #[case("pi()+100000000", "π+100000000")]
    #[case("pi()+1000000000", "π+1000000000")]
    #[case("pi()+10000000000", "π+10000000000")]
    #[case("pi()+100000000000", "π+100000000000")]
    #[case("pi()+1000000000000", "π+1000000000000")]
    #[case("pi()+1000000000000000", "π+1000000000000000")]
    #[case("pi()+1000000000000000000", "π+1000000000000000000")]
    #[case("pi()+1000000000000000000000", "π+1000000000000000000000")]
    #[case("pi()+1000000000000000000000000", "π+1000000000000000000000000")]
    #[case("pi()+1000000000000000000000000000", "π+1000000000000000000000000000")]
    #[case(
        "pi()+1000000000000000000000000000000",
        "π+1000000000000000000000000000000"
    )]
    #[case("e()+10", "e+10")]
    #[case("e()+100", "e+100")]
    #[case("e()+1000", "e+1000")]
    #[case("e()+10000", "e+10000")]
    #[case("e()+100000", "e+100000")]
    #[case("e()+1000000", "e+1000000")]
    #[case("e()+10000000", "e+10000000")]
    #[case("e()+100000000", "e+100000000")]
    #[case("e()+1000000000", "e+1000000000")]
    #[case("e()+10000000000", "e+10000000000")]
    #[case("e()+100000000000", "e+100000000000")]
    #[case("e()+1000000000000", "e+1000000000000")]
    #[case("e()+1000000000000000", "e+1000000000000000")]
    #[case("e()+1000000000000000000", "e+1000000000000000000")]
    #[case("e()+1000000000000000000000", "e+1000000000000000000000")]
    #[case("e()+1000000000000000000000000", "e+1000000000000000000000000")]
    #[case("e()+1000000000000000000000000000", "e+1000000000000000000000000000")]
    #[case(
        "e()+1000000000000000000000000000000",
        "e+1000000000000000000000000000000"
    )]
    fn big_number(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, s(input.clone()), "简单表达式“{}”", input);
    }

    #[rstest] // TODO 位数较少时转换为分数，较多时被舍弃
    #[case("pi()+0.1", "π+0.1")]
    #[case("pi()+0.01", "π+0.01")]
    #[case("pi()+0.001", "π+0.001")]
    #[case("pi()+0.0001", "π+0.0001")]
    #[case("pi()+0.00001", "π+0.00001")]
    #[case("pi()+0.000001", "π+0.000001")]
    #[case("pi()+0.0000001", "π+0.0000001")]
    #[case("pi()+0.00000001", "π+0.00000001")]
    #[case("pi()+0.000000001", "π+0.000000001")]
    #[case("pi()+0.0000000001", "π+0.0000000001")]
    #[case("pi()+0.00000000001", "π+0.00000000001")]
    #[case("pi()+0.000000000001", "π+0.000000000001")]
    #[case("pi()+0.000000000000001", "π+0.000000000000001")]
    #[case("pi()+0.000000000000000001", "π+0.000000000000000001")]
    #[case("pi()+0.000000000000000000001", "π+0.000000000000000000001")]
    #[case("pi()+0.000000000000000000000001", "π+0.000000000000000000000001")]
    #[case(
        "pi()+0.000000000000000000000000001",
        "π+0.000000000000000000000000001"
    )]
    #[case(
        "pi()+0.000000000000000000000000000001",
        "π+0.000000000000000000000000000001"
    )]
    #[case("e()+0.1", "e+0.1")]
    #[case("e()+0.01", "e+0.01")]
    #[case("e()+0.001", "e+0.001")]
    #[case("e()+0.0001", "e+0.0001")]
    #[case("e()+0.00001", "e+0.00001")]
    #[case("e()+0.000001", "e+0.000001")]
    #[case("e()+0.0000001", "e+0.0000001")]
    #[case("e()+0.00000001", "e+0.00000001")]
    #[case("e()+0.000000001", "e+0.000000001")]
    #[case("e()+0.0000000001", "e+0.0000000001")]
    #[case("e()+0.00000000001", "e+0.00000000001")]
    #[case("e()+0.000000000001", "e+0.000000000001")]
    #[case("e()+0.000000000000001", "e+0.000000000000001")]
    #[case("e()+0.000000000000000001", "e+0.000000000000000001")]
    #[case("e()+0.000000000000000000001", "e+0.000000000000000000001")]
    #[case("e()+0.000000000000000000000001", "e+0.000000000000000000000001")]
    #[case("e()+0.000000000000000000000000001", "e+0.000000000000000000000000001")]
    #[case(
        "e()+0.000000000000000000000000000001",
        "e+0.000000000000000000000000000001"
    )]
    fn small_number(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, s(input.clone()), "简单表达式“{}”", input);
    }
}
