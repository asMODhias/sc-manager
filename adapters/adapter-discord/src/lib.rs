//! Discord adapter (scoped, no business logic)

use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use std::path::Path;

/// Conservative record for Discord chat exports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscordMessage {
    pub timestamp: Option<String>,
    pub author: Option<String>,
    pub channel: Option<String>,
    pub content: String,
    pub raw_line: String,
}

/// Try to parse a single line into a `DiscordMessage`.
/// Conservative heuristics to avoid false claims.
pub fn parse_line(line: &str) -> Option<DiscordMessage> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }

    let mut timestamp: Option<String> = None;
    let mut rest = s;

    // Bracketed timestamp: [2025-12-29T12:00:00Z] #author: message
    if let Some(stripped) = s.strip_prefix('[') {
        if let Some(idx) = stripped.find(']') {
            timestamp = Some(stripped[..idx].trim().to_string());
            rest = stripped[idx + 1..].trim();
        }
    }

    // Attempt to split into author and message: 'Author#1234: message' or 'Author: message'
    let (author, content) = if let Some(colon_pos) = rest.find(':') {
        let potential_author = rest[..colon_pos].trim();
        let after = rest[colon_pos + 1..].trim();
        (Some(potential_author.to_string()), after.to_string())
    } else {
        (None, rest.to_string())
    };

    // Channel heuristics (e.g., "[#general] Author: message")
    let (channel, content) = if let Some(_ch_start) = content.strip_prefix('[') {
        if let Some(ch_end) = content.find(']') {
            let ch = content[..ch_end].to_string();
            let remainder = content[ch_end + 1..].trim().to_string();
            (Some(ch), remainder)
        } else {
            (None, content)
        }
    } else {
        (None, content)
    };

    Some(DiscordMessage {
        timestamp,
        author,
        channel,
        content: content.clone(),
        raw_line: s.to_string(),
    })
}

/// Parse all lines from a `BufRead` into Vec<DiscordMessage>
pub fn parse_reader<R: BufRead>(reader: R) -> io::Result<Vec<DiscordMessage>> {
    let mut out = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        if let Some(rec) = parse_line(&line) {
            out.push(rec);
        }
    }
    Ok(out)
}

/// Convenience for parsing files
pub fn parse_file(path: &Path) -> io::Result<Vec<DiscordMessage>> {
    let f = std::fs::File::open(path)?;
    let reader = io::BufReader::new(f);
    parse_reader(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_line_with_timestamp_and_author() {
        let line = "[2025-12-29T12:00:00Z] Alice#1234: Hello world";
        let rec = parse_line(line).expect("should parse");
        assert_eq!(rec.timestamp.unwrap(), "2025-12-29T12:00:00Z");
        assert_eq!(rec.author.unwrap(), "Alice#1234");
        assert_eq!(rec.content, "Hello world");
    }

    #[test]
    fn parse_reader_basic() {
        let data = "Alice: hi\nBob: hello\n";
        let reader = Cursor::new(data);
        let recs = parse_reader(reader).expect("parse reader");
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].author.as_deref(), Some("Alice"));
        assert_eq!(recs[1].author.as_deref(), Some("Bob"));
    }
}
