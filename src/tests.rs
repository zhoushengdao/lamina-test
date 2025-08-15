#[cfg(test)]
use crate::run;

#[test]
fn test_1_plus_1() {
    assert_eq!(run("print(1 + 1);"), "2");
}

#[test]
fn test_0_plus_0() {
    assert_eq!(run("print(0 + 0);"), "0");
}
