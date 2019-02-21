fn is_non_brace(a: char) -> bool {
    a != '{' && a != '}'
}

named!(content<&str, &str>, take_while!(is_non_brace));

named!(balanced_braces<&str, &str>,
    complete!(
        recognize!(
            delimited!(
                char!('{'),
                tuple!(
                    opt!(content),
                    opt!(balanced_braces),
                    opt!(content)
                ),
                char!('}')
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use crate::example::balanced_braces;
    use nom::Err::{Error};
    use nom::ErrorKind::{Char, Complete};
    use nom::Context::Code;
    #[test]
    fn test_brace_matching() {

        let provider = vec![
            "{inner}",
            "{a{b}a}",
            "{a{b{c}}a}",
            //"{a{b}{b}{b}a}", <==== repeated brace case does not work yet
        ];

        for data in provider {
            assert_eq!(Ok(("", data)), balanced_braces(data));
        }

        assert_eq!(Err(Error(Code("a{b}a}", Char))), balanced_braces("a{b}a}"));
        assert_eq!(Err(Error(Code("{a{ba}", Complete))), balanced_braces("{a{ba}"));
        assert_eq!(Err(Error(Code("{a{b}a", Complete))), balanced_braces("{a{b}a"));
    }
}
