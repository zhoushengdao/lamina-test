#[test]
fn test_basic_arithmetic() {
    let script = r#"
        println(2 + 3 * 4);
        println((2 + 3) * 4);
        println(10 / 3);
    "#;

    let output = run_lamina_script(script);
    let expected = "14\n20\n10/3";
    assert_eq!(output, expected);
}

#[test]
fn test_fraction_operations() {
    let script = r#"
        val a = 1/3;
        val b = 2/5;
        println(a + b);
        println(a * b);
        println(a / b);
    "#;

    let output = run_lamina_script(script);
    let expected = "11/15\n2/15\n5/6";
    assert_eq!(output, expected);
}

#[test]
fn test_irrational_numbers() {
    let script = r#"
        println(√2 * √2);
        println(π);
        println(e);
    "#;

    let output = run_lamina_script(script);
    assert!(output.contains("2")); // √2 * √2 = 2
    assert!(output.contains("π")); // 应保留符号表示
    assert!(output.contains("e")); // 应保留符号表示
}

#[test]
fn test_vector_operations() {
    let script = r#"
        val v1 = [1, 2, 3];
        val v2 = [4, 5, 6];
        println(v1 + v2);
        println(v1 · v2);  // 点积
    "#;

    let output = run_lamina_script(script);
    assert!(output.contains("[5, 7, 9]")); // 向量加法
    assert!(output.contains("32")); // 点积结果
}

#[test]
fn test_recursion() {
    let script = r#"
        fn factorial(n) {
            if n <= 1 { 1 } 
            else { n * factorial(n-1) }
        }
        println(factorial(5));
    "#;

    let output = run_lamina_script(script);
    assert_eq!(output, "120");
}

#[test]
fn test_large_integers() {
    let script = r#"
        val big = 12345678901234567890;
        println(big * big);
    "#;

    let output = run_lamina_script(script);
    let expected = "152415787532388367501905199875019052100";
    assert_eq!(output, expected);
}

#[test]
fn test_matrix_operations() {
    let script = r#"
        val m1 = [[1, 2], [3, 4]];
        val m2 = [[5, 6], [7, 8]];
        println(m1 + m2);
        println(m1 * m2);
    "#;

    let output = run_lamina_script(script);
    assert!(output.contains("[[6, 8], [10, 12]]")); // 矩阵加法
    assert!(output.contains("[[19, 22], [43, 50]]")); // 矩阵乘法
}

#[test]
fn test_precision_preservation() {
    let script = r#"
        val a = 1/3;
        val b = 1/3;
        val result = 0;
        
        for i in 1..1000 {
            result = result + a;
        }
        
        println(result);
        println(b * 1000);
    "#;

    let output = run_lamina_script(script);
    let expected = "1000/3\n1000/3";
    assert_eq!(output, expected);
}
