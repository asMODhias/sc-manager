use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use std::path::Path;

/// Parsed erkul record (conservative)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErkulRecord {
    pub timestamp: Option<String>,
    pub member_rsi: Option<String>,
    pub event: Option<String>,
    pub details: Option<String>,
    pub raw_line: String,
}

/// Parse a single line. Expected (conservative) CSV-like format:
/// timestamp,member_rsi,event,details
pub fn parse_line(line: &str) -> Option<ErkulRecord> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }

    // Split into at most 4 parts
    let parts = s.splitn(4, ',').map(|p| p.trim()).collect::<Vec<_>>();

    let timestamp = parts.first().and_then(|t| if !t.is_empty() { Some(t.to_string()) } else { None });
    let member_rsi = parts.get(1).and_then(|m| if !m.is_empty() { Some(m.to_string()) } else { None });
    let event = parts.get(2).and_then(|e| if !e.is_empty() { Some(e.to_string()) } else { None });
    let details = parts.get(3).and_then(|d| if !d.is_empty() { Some(d.to_string()) } else { None });

    Some(ErkulRecord {
        timestamp,
        member_rsi,
        event,
        details,
        raw_line: s.to_string(),
    })
}

pub fn parse_reader<R: BufRead>(reader: R) -> io::Result<Vec<ErkulRecord>> {
    let mut out = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        if let Some(rec) = parse_line(&line) {
            out.push(rec);
        }
    }
    Ok(out)
}

pub fn parse_file(path: &Path) -> io::Result<Vec<ErkulRecord>> {
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
        let line = "2025-12-29T12:00:00Z,SomeMember,OP_COMPLETE,details here";
        let rec = parse_line(line).expect("should parse");
        assert_eq!(rec.timestamp.unwrap(), "2025-12-29T12:00:00Z");
        assert_eq!(rec.member_rsi.unwrap(), "SomeMember");
        assert_eq!(rec.event.unwrap(), "OP_COMPLETE");
        assert_eq!(rec.details.unwrap(), "details here");
    }

    #[test]
    fn parse_reader_basic() {
        let data = "2025-12-29T12:00:00Z, A, EVT, 1\njust random line\n";
        let reader = Cursor::new(data);
        let recs = parse_reader(reader).expect("parse reader");
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].member_rsi.as_deref(), Some("A"));
        assert_eq!(recs[1].raw_line, "just random line");
    }
}
