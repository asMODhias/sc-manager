use proptest::prelude::*;
use adapter_gamelog::parser::parse_mission_suggestion;

proptest! {
    #[test]
    fn fuzz_lines_dont_crash(s in ".{0,256}") {
        // The parser should never panic on arbitrary input
        let _ = parse_mission_suggestion(&s);
    }
}
