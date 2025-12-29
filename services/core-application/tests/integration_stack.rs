use std::env;
use std::io::Write;

fn run_if_integration_enabled() -> bool {
    matches!(env::var("RUN_INTEGRATION_TESTS"), Ok(v) if v == "1")
}

#[test]
fn db_connect_and_crud() {
    if !run_if_integration_enabled() {
        eprintln!("Skipping DB integration test. Set RUN_INTEGRATION_TESTS=1 to run.");
        return;
    }

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/sc_manager".into());
    // Use postgres blocking client for simplicity
    let mut client = postgres::Client::connect(&db_url, postgres::NoTls).expect("connect db");

    // Create table, insert, select, cleanup
    client.execute("CREATE TABLE IF NOT EXISTS it_orgs (id TEXT PRIMARY KEY, name TEXT)", &[]).unwrap();
    client.execute("INSERT INTO it_orgs (id,name) VALUES ($1,$2) ON CONFLICT (id) DO NOTHING", &[&"org-int-1", &"IntegrationOrg"]).unwrap();
    let rows = client.query("SELECT name FROM it_orgs WHERE id = $1", &[&"org-int-1"]).unwrap();
    assert_eq!(rows.len(), 1);
    let name: String = rows[0].get(0);
    assert_eq!(name, "IntegrationOrg");
    client.execute("DROP TABLE it_orgs", &[]).unwrap();
}

#[test]
fn nats_pub_sub_roundtrip() {
    if !run_if_integration_enabled() {
        eprintln!("Skipping NATS integration test. Set RUN_INTEGRATION_TESTS=1 to run.");
        return;
    }

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    // Minimal raw-TCP NATS client to avoid extra crates in integration tests
    let url = url::Url::parse(&nats_url).expect("parse nats url");
    let host = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port_or_known_default().unwrap_or(4222);
    let addr = format!("{}:{}", host, port);

    // Subscriber
    std::thread::spawn({
        let addr = addr.clone();
        move || {
            use std::io::{Read, Write};
            let mut s = std::net::TcpStream::connect(&addr).expect("connect sub");
            s.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
            s.write_all(b"CONNECT {}\r\nSUB it.events.test 1\r\nPING\r\n").unwrap();
            // read response lines until we find MSG
            let mut buf = [0u8; 1024];
            let n = s.read(&mut buf).expect("read");
            let got = String::from_utf8_lossy(&buf[..n]).to_string();
            assert!(got.contains("MSG it.events.test"));
            assert!(got.contains("it-test"));
        }
    });

    // Publisher
    std::thread::sleep(std::time::Duration::from_millis(50));
    let mut ps = std::net::TcpStream::connect(&addr).expect("connect pub");
    ps.write_all(b"CONNECT {}\r\n").unwrap();
    let payload = "{\"event\":\"it-test\"}";
    let pub_cmd = format!("PUB it.events.test {}\r\n{}\r\n", payload.len(), payload);
    ps.write_all(pub_cmd.as_bytes()).unwrap();
}
