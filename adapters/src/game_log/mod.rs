//! Game.log parser adapter (inbound)

use sc_manager_core::domain::game_event::GameEventType;
use sc_manager_core::events::EventEnvelope;

pub trait GameLogParser {
    fn parse_line(&self, line: &str) -> Option<EventEnvelope>;
}

pub struct SimpleGameLogParser;

impl SimpleGameLogParser {
    fn parse_tokens(line: &str) -> Option<(GameEventType, i64, Option<String>)> {
        // Robust regex-based parser
        // Expected general format: "EVENT 1610000000 [key=value ...] [free text]"
        let re =
            regex::Regex::new(r"^(?P<ev>[A-Za-z_-]+)\s+(?P<ts>\d+)(?:\s+(?P<rest>.*))?$").unwrap();
        let caps = re.captures(line.trim())?;
        let ev = caps.name("ev")?.as_str().to_lowercase();
        let ts = caps.name("ts")?.as_str().parse::<i64>().ok()?;
        let rest = caps.name("rest").map(|m| m.as_str().to_string());

        // normalize event type
        let event_type = match ev.as_str() {
            "sessionstart" | "session_start" | "session-start" => GameEventType::SessionStart,
            "sessionend" | "session_end" | "session-end" => GameEventType::SessionEnd,
            "kill" => GameEventType::Kill,
            "death" => GameEventType::Death,
            _ => return None,
        };

        // parse rest into key=value tokens when present
        let details = rest
            .and_then(|r| {
                // split by whitespace, keep quoted values together
                let token_re =
                    regex::Regex::new(r#"(?P<k>[^\s=]+)=(?P<v>"[^"]+"|[^\s]+)|(?P<raw>[^\s]+)"#)
                        .unwrap();
                let mut pairs = vec![];
                for cap in token_re.captures_iter(&r) {
                    if let Some(k) = cap.name("k") {
                        let v = cap
                            .name("v")
                            .map(|m| m.as_str().trim_matches('"').to_string())
                            .unwrap_or_default();
                        pairs.push(format!("{}={}", k.as_str(), v));
                    } else if let Some(raw) = cap.name("raw") {
                        pairs.push(raw.as_str().to_string());
                    }
                }
                if pairs.is_empty() {
                    None
                } else {
                    Some(pairs.join(" "))
                }
            });

        Some((event_type, ts, details))
    }
}

impl GameLogParser for SimpleGameLogParser {
    fn parse_line(&self, line: &str) -> Option<EventEnvelope> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }
        if let Some((event_type, ts, details)) = Self::parse_tokens(trimmed) {
            // id can include event and timestamp
            let id = format!("evt-{}-{}", event_type as i32, ts);
            return Some(EventEnvelope::GameEvent {
                id,
                event_type,
                timestamp: ts,
                details,
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_manager_core::domain::game_event::GameEventType;

    #[test]
    fn parse_session_start() {
        let p = SimpleGameLogParser;
        let line = "SessionStart 1610000000";
        let out = p.parse_line(line).expect("should parse");
        match out {
            EventEnvelope::GameEvent {
                event_type,
                timestamp,
                ..
            } => {
                assert_eq!(event_type, GameEventType::SessionStart);
                assert_eq!(timestamp, 1610000000);
            }
            _ => panic!("unexpected envelope type"),
        }
    }

    #[test]
    fn parse_kill_with_details() {
        let p = SimpleGameLogParser;
        let line = "Kill 1610000100 killer=alice victim=bob";
        let out = p.parse_line(line).expect("should parse kill");
        match out {
            EventEnvelope::GameEvent {
                event_type,
                timestamp,
                ..
            } => {
                assert_eq!(event_type, GameEventType::Kill);
                assert_eq!(timestamp, 1610000100);
            }
            _ => panic!("unexpected envelope type"),
        }
    }
}
