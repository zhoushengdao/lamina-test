# Lamina 测试

[![Test Lamina](https://github.com/zhoushengdao/lamina-test/actions/workflows/test.yml/badge.svg)](https://github.com/zhoushengdao/lamina-test/actions/workflows/test.yml)

测试用例：[src/tests.rs](src/tests.rs)

测试结果：[最新测试结果](https://github.com/zhoushengdao/lamina-test/issues/1#issuecomment-3192037735)

## 贡献

> 为 Lamina 贡献：请看这里：[Lamina-dev/Lamina: 一种专注于精确数学计算的面向过程编程语言](https://github.com/Lamina-dev/Lamina/)

为本项目作贡献：

> 推荐阅读：[Writing Automated Tests - The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)、[Tests - The rustc book](https://doc.rust-lang.org/rustc/tests/index.html)、[Testing - The Rust Reference](https://doc.rust-lang.org/reference/attributes/testing.html)、[test - The Rust Unstable Book](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

1. 克隆仓库：`git clone https://github.com/zhoushengdao/lamina-test.git`
2. 创建并切换分支：`git checkout -b <name>`
3. 在 `src/tests.rs` 中编写新的测试。  
   考虑到目前主要是测试同一功能在不同参数下的结果，所以请仿照下面的示例编写测试。

   ```rust
   #[rstest]
   #[case("pi()", "π")]
   #[case("e()", "e")]
   #[case("true", "true")]
   #[case("false", "false")]
   #[case("null", "null")]
   #[case("\"str\"", "str")]
   fn print_object(#[case] input: String, #[case] expected: String) {
      assert_eq!(expected, s(input.clone()), "简单表达式“{}”", input);
   }
   ```

   - 其中，每一个 `case` 都对应一组输入和预期的输出。参见 [rstest in rstest - Rust](https://docs.rs/rstest/latest/rstest/attr.rstest.html)。
   - 函数名要有意义，方便查阅。
   - 函数的输入参数应该保留原样。
   - `assert_eq!` 用来比较预期值和实际值是否相等。参见 [std - Rust](https://doc.rust-lang.org/std/index.html#macros)。
   - `s(input.clone())` 中的 `s` 函数是为了方便执行单行语句而定义的工具函数，给它传递的字符串会被自动用 `print();` 包裹，再用 Lamina 运行。因此你就无需写 `print(1+1);`，而只需写 `1+1`即可。
   - 与 `s()` 类似的是 `m()` 函数，它用于处理多行语句。其内容会被原样输入给 Lamina 来运行。
   - 最后的 `"简单表达式“{}”", input` 用来提供自定义错误信息，方便标识出错的错误用例。参见 [assert in std - Rust](https://doc.rust-lang.org/std/macro.assert.html#custom-messages)。
