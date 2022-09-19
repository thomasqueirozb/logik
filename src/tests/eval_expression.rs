use crate::parser::eval_expression;

#[test]
fn comment() {
    assert_eq!(eval_expression("/* a */ 1 /* b */").unwrap(), 1);
    assert_eq!(eval_expression("/* /* 3 */ 1 /* b */").unwrap(), 1);
    assert_eq!(eval_expression("/* /**/ 2+1 /* b */").unwrap(), 3);
    assert_eq!(eval_expression("1-/*/**/ 2+1 /* b */").unwrap(), 0);
    assert_eq!(eval_expression("2*/*/**/ 3+5 /* b */").unwrap(), 11);
    assert_eq!(eval_expression("2 /* * /+ 2 */ + 2").unwrap(), 4);
}

#[test]
fn parenthesis() {
    assert_eq!(eval_expression("---(1) + 4 * (3+5)").unwrap(), 31);
    assert!(eval_expression("---(1) + 4 * (3+5))").is_err());
    assert_eq!(
        eval_expression("(3*2) * 4 * (5*(3+(2*1+1))) * 7").unwrap(),
        5040
    );
    assert!(eval_expression("1()").is_err());
    assert!(eval_expression("1(+)").is_err());
}

#[test]
fn sum_sub() {
    assert_eq!(eval_expression("1  -   04 ").unwrap(), -3);
    assert_eq!(eval_expression("11-4").unwrap(), 7);
    assert_eq!(eval_expression("11-4 + 22 -23").unwrap(), 6);
    assert_eq!(eval_expression("1  +   4 ").unwrap(), 5);
    assert_eq!(eval_expression("1 + 40 ").unwrap(), 41);
}

#[test]
fn sub() {
    assert_eq!(eval_expression("1  -   04 - 10").unwrap(), -13);
    assert_eq!(eval_expression("11-4-7").unwrap(), 0);
    assert_eq!(eval_expression("-3").unwrap(), -3);
    assert_eq!(eval_expression("--3").unwrap(), 3);
    assert_eq!(eval_expression("---3").unwrap(), -3);
}

#[test]
fn sum() {
    assert_eq!(eval_expression("1  +   04 + 10").unwrap(), 15);
    assert_eq!(eval_expression("11+4+7").unwrap(), 22);
}

#[test]
fn div_mul() {
    assert_eq!(eval_expression("81/ 9 + 3").unwrap(), 12);
    assert_eq!(eval_expression("4/2").unwrap(), 2);
    assert_eq!(eval_expression("4/2 + 3").unwrap(), 5);
    assert_eq!(eval_expression("3 + 4/2").unwrap(), 5);
    assert_eq!(eval_expression("3*5 + 10").unwrap(), 25);
    assert_eq!(eval_expression("10 + 3*5").unwrap(), 25);
}

#[test]
fn errors() {
    assert_eq!(
        eval_expression("1 a  -   04 ").unwrap_err().to_string(),
        "Finished parsing but not EOF"
    );
    assert_eq!(
        eval_expression("1a1-4").unwrap_err().to_string(),
        "Finished parsing but not EOF"
    );

    assert_eq!(
        eval_expression("11-4/* + 22 -23").unwrap_err().to_string(),
        "Unterminated comment"
    );
    assert_eq!(
        eval_expression("3- 3 /* a").unwrap_err().to_string(),
        "Unterminated comment"
    );

    assert_eq!(
        eval_expression("3+ /* a */").unwrap_err().to_string(),
        "Expected number, variable, operator or '(', found EOF @ 0:11"
    );

    assert_eq!(
        eval_expression("3+ /* a */-").unwrap_err().to_string(),
        "Expected number, variable, operator or '(', found EOF @ 0:12"
    );

    assert_eq!(
        eval_expression("/* */").unwrap_err().to_string(),
        "Expected number, variable, operator or '(', found EOF @ 0:6"
    );
    assert_eq!(
        eval_expression("/* 1 + 1*/").unwrap_err().to_string(),
        "Expected number, variable, operator or '(', found EOF @ 0:11"
    );

    assert_eq!(
        eval_expression("*/**/").unwrap_err().to_string(),
        "Expected '+' or '-' or '!' found '*'"
    );
}
