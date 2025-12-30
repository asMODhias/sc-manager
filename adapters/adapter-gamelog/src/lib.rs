pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::parse_game_log_line;

    #[test]
    fn test_parse_simple_event() {
        let line = "2025-12-27T12:00:00Z EVENT: PLAYER_JOIN id=player1";
        let evt = parse_game_log_line(line).expect("parse failed");
        assert_eq!(evt.event_type, "EVENT:");
        assert_eq!(evt.details.as_deref().unwrap_or_default(), "PLAYER_JOIN id=player1");
    }
}
