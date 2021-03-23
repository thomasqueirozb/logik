#[cfg(test)]
mod test {
    use crate::parse::eval;

    #[test]
    fn comment() {
        assert_eq!(eval("/* a */ 1 /* b */").unwrap(), 1);
        assert_eq!(eval("/* /* 3 */ 1 /* b */").unwrap(), 1);
        assert_eq!(eval("/* /**/ 2+1 /* b */").unwrap(), 3);
        assert_eq!(eval("1-/*/**/ 2+1 /* b */").unwrap(), 0);
        assert_eq!(eval("2*/*/**/ 3+5 /* b */").unwrap(), 11);
        assert_eq!(eval("2 /* * /+ 2 */ + 2").unwrap(), 4);
    }

    #[test]
    fn sum_sub() {
        assert_eq!(eval("1  -   04 ").unwrap(), -3);
        assert_eq!(eval("11-4").unwrap(), 7);
        assert_eq!(eval("11-4 + 22 -23").unwrap(), 6);
        assert_eq!(eval("1  +   4 ").unwrap(), 5);
        assert_eq!(eval("1 + 40 ").unwrap(), 41);
    }

    #[test]
    fn div_mul() {
        assert_eq!(eval("81/ 9 + 3").unwrap(), 12);
        assert_eq!(eval("4/2").unwrap(), 2);
        assert_eq!(eval("4/2 + 3").unwrap(), 5);
        assert_eq!(eval("3 + 4/2").unwrap(), 5);
        assert_eq!(eval("3*5 + 10").unwrap(), 25);
        assert_eq!(eval("10 + 3*5").unwrap(), 25);
    }

    #[test]
    fn errors() {
        assert_eq!(
            eval("1 a  -   04 ").unwrap_err().to_string(),
            "Unparsable char 'a'"
        );
        assert_eq!(
            eval("1a1-4").unwrap_err().to_string(),
            "Could not convert \"1a1\" to i64 - (invalid digit found in string)"
        );
        assert_eq!(
            eval("11-4/* + 22 -23").unwrap_err().to_string(),
            "Unterminated comment"
        );
        assert_eq!(
            eval("3- 3 /* a").unwrap_err().to_string(),
            "Unterminated comment"
        );

        assert_eq!(
            eval("3+ /* a */").unwrap_err().to_string(),
            "Expected number after operator"
        );

        assert_eq!(
            eval("-3 /* a */").unwrap_err().to_string(),
            "Expected number found operator \"-\""
        );

        assert_eq!(
            eval("3+ /* a */-").unwrap_err().to_string(),
            "Expected number found operator \"-\""
        );

        assert_eq!(eval("/* */").unwrap_err().to_string(), "No tokens found");
        assert_eq!(
            eval("/* 1 + 1*/").unwrap_err().to_string(),
            "No tokens found"
        );
    }
}
