pub struct GameEvent {
    pub timestamp: String,
    pub event_type: String,
    pub details: Option<String>,
}

pub fn parse_game_log_line(line: &str) -> Result<GameEvent, &'static str> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err("invalid format");
    }
    Ok(GameEvent {
        timestamp: parts[0].to_string(),
        event_type: parts[1].to_string(),
        details: Some(parts[2].trim().to_string()),
    })
}

// --- Mission suggestion parsing (conservative, ToS-safe) ---
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct MissionCompletionSuggestion {
    pub mission_name: String,
    pub member_rsi: Option<String>,
    pub timestamp: String,
    pub raw_line: String,
}

/// Try to parse a mission completion suggestion. Conservative: only return Some for
/// clearly matching lines.
pub fn parse_mission_suggestion(line: &str) -> Option<MissionCompletionSuggestion> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<ts>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z).*Member\s+(?P<rsi>[A-Za-z0-9_\-]+)\s+completed\s+mission\s+(?P<mission>.+)").unwrap();
    }

    if let Some(caps) = RE.captures(line) {
        let mission = caps.name("mission").map(|m| m.as_str().trim().to_string()).unwrap_or_default();
        let rsi = caps.name("rsi").map(|m| m.as_str().to_string());
        let ts = caps.name("ts").map(|m| m.as_str().to_string()).unwrap_or_default();

        return Some(MissionCompletionSuggestion {
            mission_name: mission,
            member_rsi: rsi,
            timestamp: ts,
            raw_line: line.to_string(),
        });
    }

    None
}

#[cfg(test)]
mod mission_tests {
    use super::*;

    #[test]
    fn parse_valid_mission_line() {
        let line = "2025-12-27T20:34:12Z - Member Alpha_One completed mission Wikelo Delivery";
        let s = parse_mission_suggestion(line).expect("Should parse");
        assert_eq!(s.mission_name, "Wikelo Delivery");
        assert_eq!(s.member_rsi, Some("Alpha_One".to_string()));
        assert_eq!(s.timestamp, "2025-12-27T20:34:12Z");
    }

    #[test]
    fn ignore_non_mission_line() {
        let line = "2025-12-27T20:34:12Z - Player joined the instance";
        assert!(parse_mission_suggestion(line).is_none());
    }
}
