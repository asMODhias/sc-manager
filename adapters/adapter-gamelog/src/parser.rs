use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::fs::File;

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

#[derive(Debug, PartialEq, Eq, Serialize)]
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

/// Parse a reader line-by-line and collect suggestions. Streaming-friendly.
pub fn parse_reader<R: BufRead>(reader: R) -> Vec<MissionCompletionSuggestion> {
    let mut suggestions = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        if let Some(s) = parse_mission_suggestion(&line) {
            suggestions.push(s);
        }
    }
    suggestions
}

/// Parse a file and return suggestions.
pub fn parse_file(path: &std::path::Path) -> Result<Vec<MissionCompletionSuggestion>, std::io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    Ok(parse_reader(reader))
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

    #[test]
    fn parse_file_with_multiple_lines() {
        let mut tmp = std::env::temp_dir();
        tmp.push("adapter_gamelog_test_sample.log");
        std::fs::write(&tmp, "2025-12-27T20:34:12Z - Member Alpha_One completed mission Wikelo Delivery\n2025-12-27T20:35:00Z - Player joined the instance\n2025-12-27T20:40:00Z - Member Beta completed mission Mining Run\n").unwrap();

        let res = parse_file(&tmp).expect("file read");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].mission_name, "Wikelo Delivery");
        assert_eq!(res[1].mission_name, "Mining Run");

        let _ = std::fs::remove_file(&tmp);
    }
}

