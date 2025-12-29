use std::env;
use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::thread;

fn run_if_integration_enabled() -> bool {
    match env::var("RUN_INTEGRATION_TESTS") {
        Ok(v) if v == "1" => true,
        _ => false,
    }
}

#[test]
fn nats_to_postgres_end_to_end() {
    if !run_if_integration_enabled() {
        eprintln!("Skipping NATS→Postgres E2E test. Set RUN_INTEGRATION_TESTS=1 to run.");
        return;
    }

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/sc_manager".into());

    // parse host:port
    let url = url::Url::parse(&nats_url).expect("parse nats url");
    let host = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port_or_known_default().unwrap_or(4222);
    let addr = format!("{}:{}", host, port);

    // channel to notify main thread that subscriber wrote to DB
    let (tx, rx) = channel();

    // Spawn subscriber thread that listens for the subject and writes to Postgres
    let subject = "it.events.e2e".to_string();
    let db_url_clone = db_url.clone();
    let addr_clone = addr.clone();
    let subject_clone = subject.clone();
    let subject_for_db = subject.clone();
    let sub_handle = thread::spawn(move || {
        let mut s = std::net::TcpStream::connect(&addr_clone).expect("connect sub");
        s.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
        s.write_all(b"CONNECT {}\r\n").unwrap();
        let sub_cmd = format!("SUB {} 1\r\n", subject_clone);
        s.write_all(sub_cmd.as_bytes()).unwrap();
        s.write_all(b"PING\r\n").unwrap();

        let mut buf = vec![0u8; 4096];
        let mut accumulated = Vec::new();

        loop {
            match s.read(&mut buf) {
                Ok(n) if n == 0 => break,
                Ok(n) => {
                    accumulated.extend_from_slice(&buf[..n]);
                    if let Ok(text) = std::str::from_utf8(&accumulated) {
                        // helpful debug for flaky container runs
                        if text.contains("PUB") || text.contains("MSG") || text.contains("e2e-test") {
                            eprintln!("subscriber saw (truncated): {}", &text.chars().take(500).collect::<String>());
                        }

                        // payload we expect - robust NATS MSG framing parse
                        if text.contains("\"event\":\"e2e-test\"") || text.contains("MSG ") {
                            // attempt to parse MSG header and read exact payload bytes
                            if let Some(msg_pos) = text.find("MSG ") {
                                // find end of header (first \r\n after msg_pos)
                                if let Some(rel_header_end) = text[msg_pos..].find("\r\n") {
                                    let header_end = msg_pos + rel_header_end;
                                    let header = &text[msg_pos..header_end];
                                    eprintln!("found MSG header: {:?}", header);
                                    // header format: MSG <subject> <sid> <len>
                                    let parts: Vec<&str> = header.split_whitespace().collect();
                                    if parts.len() >= 4 {
                                        if let Ok(len) = parts[3].parse::<usize>() {
                                            let payload_start = header_end + 2; // after \r\n
                                            let needed = payload_start + len + 2; // payload + trailing \r\n
                                            if accumulated.len() >= needed {
                                                // we have full payload
                                                let payload_bytes = &accumulated[payload_start..payload_start+len];
                                                if let Ok(payload) = std::str::from_utf8(payload_bytes) {
                                                    eprintln!("extracted payload: {:?}", &payload.chars().take(200).collect::<String>());
                                                    // write to Postgres
                                                    let mut client = postgres::Client::connect(&db_url_clone, postgres::NoTls).expect("connect db");
                                                    client.execute("CREATE TABLE IF NOT EXISTS it_events (id SERIAL PRIMARY KEY, subj TEXT, payload TEXT)", &[]).unwrap();
                                                    client.execute("INSERT INTO it_events (subj, payload) VALUES ($1,$2)", &[&subject_for_db, &payload]).unwrap();
                                                    tx.send(()).unwrap();
                                                    break;
                                                } else {
                                                    eprintln!("payload not utf8");
                                                }
                                            } else {
                                                eprintln!("awaiting payload bytes: have {}, need {}", accumulated.len(), needed);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("read error: {}", e);
                }
            }
        }
    });

    // Give subscriber a short moment to set up (increase wait for containerized NATS)
    eprintln!("waiting for subscriber to register...");
    thread::sleep(std::time::Duration::from_millis(2000));

    // Publisher: send twice with small delay to reduce flakiness
    eprintln!("publishing test event to subject {}", subject);
    let mut ps = std::net::TcpStream::connect(&addr).expect("connect pub");
    ps.write_all(b"CONNECT {}\r\n").unwrap();
    let payload = "{\"event\":\"e2e-test\",\"id\":\"evt-e2e-1\",\"data\":\"hello\"}";
    let pub_cmd = format!("PUB {} {}\r\n{}\r\n", subject, payload.len(), payload);
    ps.write_all(pub_cmd.as_bytes()).unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    ps.write_all(pub_cmd.as_bytes()).unwrap();

    // wait for subscriber signal
    let ok = rx.recv_timeout(std::time::Duration::from_secs(10)).expect("subscriber should signal");
    drop(ok);

    // Verify DB contains the row
    let mut client = postgres::Client::connect(&db_url, postgres::NoTls).expect("connect db main");
    let rows = client.query("SELECT payload FROM it_events WHERE subj = $1 ORDER BY id DESC LIMIT 1", &[&subject]).unwrap();
    assert!(!rows.is_empty());
    let retrieved: String = rows[0].get(0);
    eprintln!("DB retrieved payload: {:?}", retrieved);
    assert!(retrieved.contains("e2e-test"));

    // cleanup
    client.execute("DELETE FROM it_events WHERE subj = $1", &[&subject]).unwrap();

    // join subscriber
    let _ = sub_handle.join();
}
