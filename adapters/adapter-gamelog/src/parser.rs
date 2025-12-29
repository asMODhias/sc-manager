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
