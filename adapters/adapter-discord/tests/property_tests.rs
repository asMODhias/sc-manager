use proptest::prelude::*;
use proptest::string::string_regex;
use adapter_discord::parse_line;

proptest! {
    #[test]
    fn parse_random_lines(s in string_regex(".{0,300}").unwrap()) {
        if let Some(rec) = parse_line(&s) {
            prop_assert_eq!(rec.raw_line, s.trim().to_string());
            prop_assert!(!rec.content.is_empty());
        } else {
            prop_assert!(s.trim().is_empty());
        }
    }
}
