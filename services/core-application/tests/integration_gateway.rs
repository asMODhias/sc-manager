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
fn gateway_pub_to_nats_writes_db() {
    if !run_if_integration_enabled() {
        eprintln!("Skipping gateway E2E test. Set RUN_INTEGRATION_TESTS=1 to run.");
        return;
    }

    let gateway_url = env::var("GATEWAY_URL").unwrap_or_else(|_| "http://gateway:8080".into());
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/sc_manager".into());

    // prepare subscriber to write to DB on message
    let url = url::Url::parse(&nats_url).expect("parse nats url");
    let host = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port_or_known_default().unwrap_or(4222);
    let addr = format!("{}:{}", host, port);

    let (tx, rx) = channel();
    // gateway publishes to `domain.events` subject
    let subj = "domain.events".to_string();
    let addr_clone = addr.clone();
    let subj_clone = subj.clone();
    let db_url_clone = db_url.clone();

    let sub_handle = thread::spawn(move || {
        let mut s = std::net::TcpStream::connect(&addr_clone).expect("connect sub");
        s.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
        s.write_all(b"CONNECT {}\r\n").unwrap();
        let sub_cmd = format!("SUB {} 1\r\n", subj_clone);
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
                        // robust MSG framing parse similar to other tests
                        if text.contains("\"event\":\"gateway-e2e\"") || text.contains("MSG ") {
                            if let Some(msg_pos) = text.find("MSG ") {
                                if let Some(rel_header_end) = text[msg_pos..].find("\r\n") {
                                    let header_end = msg_pos + rel_header_end;
                                    let header = &text[msg_pos..header_end];
                                    eprintln!("found MSG header: {:?}", header);
                                    let parts: Vec<&str> = header.split_whitespace().collect();
                                    if parts.len() >= 4 {
                                        if let Ok(len) = parts[3].parse::<usize>() {
                                            let payload_start = header_end + 2;
                                            let needed = payload_start + len + 2;
                                            if accumulated.len() >= needed {
                                                let payload_bytes = &accumulated[payload_start..payload_start+len];
                                                if let Ok(payload) = std::str::from_utf8(payload_bytes) {
                                                    let mut client = postgres::Client::connect(&db_url_clone, postgres::NoTls).expect("connect db");
                                                    client.execute("CREATE TABLE IF NOT EXISTS it_gateway_events (id SERIAL PRIMARY KEY, payload TEXT)", &[]).unwrap();
                                                    client.execute("INSERT INTO it_gateway_events (payload) VALUES ($1)", &[&payload]).unwrap();
                                                    tx.send(()).unwrap();
                                                    break;
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
                Err(e) => panic!("read error: {}", e),
            }
        }
    });

    // small sleep to let subscriber set up (increase for containerized NATS)
    thread::sleep(std::time::Duration::from_millis(2000));

    // POST to gateway
    let client = reqwest::blocking::Client::new();
    let payload = serde_json::json!({"event":"gateway-e2e","id":"g1","data":"hello"});

    let resp = client.post(format!("{}/events", gateway_url)).json(&payload).send().expect("post");
    assert!(resp.status().is_success());

    // wait for subscriber to signal DB write (increase timeout for CI reliability)
    rx.recv_timeout(std::time::Duration::from_secs(10)).expect("should receive db write");

    let mut client2 = postgres::Client::connect(&db_url, postgres::NoTls).expect("connect db main");
    let rows = client2.query("SELECT payload FROM it_gateway_events ORDER BY id DESC LIMIT 1", &[]).unwrap();
    assert!(!rows.is_empty());
    let fetched: String = rows[0].get(0);
    assert!(fetched.contains("gateway-e2e"));

    // cleanup
    client2.execute("DELETE FROM it_gateway_events", &[]).unwrap();

    let _ = sub_handle.join();
}
