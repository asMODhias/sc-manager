use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use std::path::Path;

/// Conservative record parsed from UEX-style log lines.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UexRecord {
    pub timestamp: Option<String>,
    pub event: String,
    pub payload: Option<String>,
    pub raw_line: String,
}

/// Try to parse a single line into a `UexRecord`.
/// This parser is intentionally conservative: if it can't reasonably
/// extract an event it still returns the raw line wrapped in a record
/// with `event = "unknown"`.
pub fn parse_line(line: &str) -> Option<UexRecord> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }

    // Naive timestamp extraction: look for a leading bracketed timestamp or ISO-like prefix
    let mut rest = s;
    let mut timestamp: Option<String> = None;

    if let Some(stripped) = s.strip_prefix('[') {
        if let Some(idx) = stripped.find(']') {
            timestamp = Some(stripped[..idx].trim().to_string());
            rest = stripped[idx + 1..].trim();
        }
    } else if let Some(idx) = s.find(' ') {
        // token before first space looks like iso timestamp?
        let first = &s[..idx];
        if first.contains('-') && first.contains(':') {
            timestamp = Some(first.to_string());
            rest = s[idx + 1..].trim();
        }
    }

    // Event/payload split heuristic: first word is event, rest is payload
    let mut parts = rest.splitn(2, ' ');
    let event = parts.next().unwrap_or("unknown").trim().to_string();
    let payload = parts.next().map(|p| p.trim().to_string()).filter(|p| !p.is_empty());

    Some(UexRecord {
        timestamp,
        event: if event.is_empty() { "unknown".to_string() } else { event },
        payload,
        raw_line: s.to_string(),
    })
}

/// Parse all lines from a `BufRead` into a Vec of `UexRecord`.
pub fn parse_reader<R: BufRead>(reader: R) -> io::Result<Vec<UexRecord>> {
    let mut out = Vec::new();

    for line_res in reader.lines() {
        let line = line_res?;
        if let Some(rec) = parse_line(&line) {
            out.push(rec);
        }
    }

    Ok(out)
}

/// Convenience for parsing a file from disk.
pub fn parse_file(path: &Path) -> io::Result<Vec<UexRecord>> {
    let f = std::fs::File::open(path)?;
    let reader = io::BufReader::new(f);
    parse_reader(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_line_basic() {
        let line = "[2025-12-29T12:00:00Z] EVENT_NAME some payload here";
        let rec = parse_line(line).expect("should parse");
        assert_eq!(rec.timestamp.unwrap(), "2025-12-29T12:00:00Z");
        assert_eq!(rec.event, "EVENT_NAME");
        assert_eq!(rec.payload.unwrap(), "some payload here");
        assert_eq!(rec.raw_line, line);
    }

    #[test]
    fn parse_reader_basic() {
        let data = "[2025-12-29T12:00:00Z] E1 p1\nE2 p2\n\n   \n";
        let reader = Cursor::new(data);
        let recs = parse_reader(reader).expect("parse reader");
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].event, "E1");
        assert_eq!(recs[1].event, "E2");
    }
}
