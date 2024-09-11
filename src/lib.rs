pub use regex_inner::*;

#[macro_export]
macro_rules! regex {
    ($re:expr) => {
        {
            static RE: std::sync::LazyLock<$crate::Regex> = std::sync::LazyLock::new(|| $crate::Regex::new($re).unwrap());
            &*RE
        }
    };
    ($re:expr, $flags:literal) => {
        // use regex builder
        {
            static RE: std::sync::LazyLock<$crate::Regex> = std::sync::LazyLock::new(|| {
                let mut r = $crate::RegexBuilder::new($re.as_ref());
                for f in $flags.chars() {
                    match f {
                        'i' => r.case_insensitive(true),
                        _ => panic!("Invalid regex flag: {}", f),
                    };
                }
                r.build().unwrap()
            });
            &*RE
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_regex() {
        let re = regex!(r#"pay\sto\sthe\sorder\sof"#);
        assert_eq!(re.is_match("pay to the order of"), true);
        let re = regex!(r#"pay\sto\sthe\sorder\sof"#, "i");
        assert_eq!(re.is_match("PAY TO THE ORDER OF"), true);
        assert_eq!(re.is_match("PAY TO THE "), false);

    }
}