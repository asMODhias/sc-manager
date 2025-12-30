use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::fs::File;
use chrono::{DateTime, Utc};

/// Minimal FleetYards CSV record representation (adapter only — no business logic)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetyardsRecord {
    pub timestamp: Option<DateTime<Utc>>,
    pub member_rsi: Option<String>,
    pub ship: Option<String>,
    pub action: Option<String>,
    pub raw_line: String,
}

pub fn name() -> &'static str { "FleetYards" }

/// Parse CSV reader into a vector of `FleetyardsRecord`.
pub fn parse_reader<R: BufRead>(reader: R) -> Vec<FleetyardsRecord> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(reader);

    let mut res = Vec::new();

    for result in rdr.records().flatten() {
        // Conservative mapping: attempt to read first 3 columns if present
        let ts = result.get(0).map(|s| s.trim().to_string());
        let member = result.get(1).map(|s| s.trim().to_string());
        let ship = result.get(2).map(|s| s.trim().to_string());
        let action = result.get(3).map(|s| s.trim().to_string());

        let parsed_ts = ts.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc)));

        res.push(FleetyardsRecord {
            timestamp: parsed_ts,
            member_rsi: member,
            ship,
            action,
            raw_line: result.iter().collect::<Vec<&str>>().join(","),
        });
    }

    res
}

/// Parse a file containing FleetYards CSV export
pub fn parse_file(path: &std::path::Path) -> Result<Vec<FleetyardsRecord>, std::io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    Ok(parse_reader(reader))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_simple_csv() {
        let data = "2025-12-27T12:00:00Z,Alpha_One,Wikelo Delivery,completed\n2025-12-27T12:05:00Z,Beta,Mining Run,completed\n";
        let cur = Cursor::new(data);
        let res = parse_reader(cur);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].member_rsi.as_deref(), Some("Alpha_One"));
        assert_eq!(res[1].ship.as_deref(), Some("Mining Run"));
    }

    #[test]
    fn parse_file_roundtrip() {
        let mut tmp = std::env::temp_dir();
        tmp.push("adapter_fleetyards_test_sample.csv");
        std::fs::write(&tmp, "2025-12-27T12:00:00Z,Alpha_One,Wikelo Delivery,completed\n").unwrap();
        let res = parse_file(&tmp).expect("file read");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].member_rsi.as_deref(), Some("Alpha_One"));
        let _ = std::fs::remove_file(&tmp);
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn fuzz_csv_lines(s in ".{0,256}") {
            // Parser should never panic on arbitrary input
            let _ = parse_reader(Cursor::new(s));
        }
    }
}

