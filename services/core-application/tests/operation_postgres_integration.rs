use std::env;
use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::thread;

#[test]
fn operation_to_postgres_e2e() {
    if env::var("RUN_INTEGRATION_TESTS").ok().as_deref() != Some("1") {
        eprintln!("Skipping operation→Postgres E2E test. Set RUN_INTEGRATION_TESTS=1 to run.");
        return;
    }

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/sc_manager".into());

    let url = url::Url::parse(&nats_url).expect("parse nats url");
    let host = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port_or_known_default().unwrap_or(4222);
    let addr = format!("{}:{}", host, port);

    let (tx, rx) = channel();

    // subscriber that writes payload to Postgres
    let subject = "domain.operations".to_string();
    let addr_clone = addr.clone();
    let db_url_clone = db_url.clone();
    let subject_clone = subject.clone();
    let sub_handle = thread::spawn(move || {
        let mut s = std::net::TcpStream::connect(&addr_clone).expect("connect sub");
        s.set_read_timeout(Some(std::time::Duration::from_secs(20))).unwrap();
        s.write_all(b"CONNECT {}\r\n").unwrap();
        let sub_cmd = format!("SUB {} 1\r\n", subject_clone);
        s.write_all(sub_cmd.as_bytes()).unwrap();
        s.write_all(b"PING\r\n").unwrap();

        let mut buf = vec![0u8; 8192];
        let mut accumulated = Vec::new();

        loop {
            match s.read(&mut buf) {
                Ok(n) if n == 0 => break,
                Ok(n) => {
                    accumulated.extend_from_slice(&buf[..n]);
                    if let Ok(text) = std::str::from_utf8(&accumulated) {
                        if text.contains("MSG") || text.contains("domain.operations") {
                            // attempt to find MSG header and payload
                            if let Some(msg_pos) = text.find("MSG ") {
                                if let Some(rel_header_end) = text[msg_pos..].find("\r\n") {
                                    let header_end = msg_pos + rel_header_end;
                                    let header = &text[msg_pos..header_end];
                                    let parts: Vec<&str> = header.split_whitespace().collect();
                                    if parts.len() >= 4 {
                                        if let Ok(len) = parts[3].parse::<usize>() {
                                            let payload_start = header_end + 2;
                                            let needed = payload_start + len + 2;
                                            if accumulated.len() >= needed {
                                                let payload_bytes = &accumulated[payload_start..payload_start + len];
                                                if let Ok(payload) = std::str::from_utf8(payload_bytes) {
                                                    eprintln!("extracted payload (truncated): {}", &payload.chars().take(200).collect::<String>());
                                                    let mut client = postgres::Client::connect(&db_url_clone, postgres::NoTls).expect("connect db");
                                                    client.execute("CREATE TABLE IF NOT EXISTS it_operations (id SERIAL PRIMARY KEY, subj TEXT, payload TEXT)", &[]).unwrap();
                                                    client.execute("INSERT INTO it_operations (subj, payload) VALUES ($1,$2)", &[&subject_clone, &payload]).unwrap();
                                                    tx.send(()).unwrap();
                                                    break;
                                                }
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

    // give subscriber time to register
    thread::sleep(std::time::Duration::from_millis(1500));

    // create signed event and publish via plain NATS TCP
    use sc_manager_app::signing::{DomainEvent, sign_event, generate_test_keypair};
    use serde_json::to_string;

    let kp = generate_test_keypair();
    let domain = DomainEvent { id: "op-e2e-1".into(), kind: "OperationCreated".into(), payload: serde_json::json!({"operation_id":"op-e2e-1","name":"E2E Ops"}) };
    let signed = sign_event(&kp, &domain);
    let ser = to_string(&signed).expect("serialize signed event");

    // connect to nats and publish
    let mut ps = std::net::TcpStream::connect(&addr).expect("connect pub");
    ps.write_all(b"CONNECT {}\r\n").unwrap();
    let pub_cmd = format!("PUB {} {}\r\n{}\r\n", subject, ser.len(), ser);
    ps.write_all(pub_cmd.as_bytes()).unwrap();

    // wait for subscriber
    let _ = rx.recv_timeout(std::time::Duration::from_secs(10)).expect("subscriber should signal");

    // verify DB
    let mut client = postgres::Client::connect(&db_url, postgres::NoTls).expect("connect db main");
    let rows = client.query("SELECT payload FROM it_operations WHERE subj = $1 ORDER BY id DESC LIMIT 1", &[&subject]).unwrap();
    assert!(!rows.is_empty());
    let retrieved: String = rows[0].get(0);
    assert!(retrieved.contains("OperationCreated"));

    // cleanup
    client.execute("DELETE FROM it_operations WHERE subj = $1", &[&subject]).unwrap();

    let _ = sub_handle.join();
}