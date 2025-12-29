use proptest::prelude::*;
use proptest::string::string_regex;
use adapter_uex::parse_line;

proptest! {
    #[test]
    fn parse_random_lines(s in string_regex(".{0,200}").unwrap()) {
        // Parsing must not panic and raw_line must equal input when parsed
        if let Some(rec) = parse_line(&s) {
            prop_assert_eq!(rec.raw_line, s.trim().to_string());
            prop_assert!(!rec.event.is_empty());
        } else {
            // Accept None for empty or whitespace-only inputs
            prop_assert!(s.trim().is_empty());
        }
    }
}
