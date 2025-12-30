RSI Adapter — Handle verification

This adapter provides a minimal interface to verify RSI handles.

Trait:
- `RsiClient::verify_handle(&self, handle: &str) -> Result<bool, String>`

In-memory test client:
- `SimpleRsiClient` accepts a list of known handles and supports `with_error(true)` to simulate network errors.

Example usage in tests:

```rust
let client = sc_manager_adapters::rsi::SimpleRsiClient::new(vec!["alice"]);
let ok = client.verify_handle("alice").unwrap();
assert!(ok);
```

Notes:
- The app uses a verification closure (e.g. `|h| client.verify_handle(h)`) so the adapter remains decoupled from application logic.
- Production adapter would implement HTTP calls and proper retries/backoff.

Example: Adapter-to-API JSON (hypothetical)

```json
{
  "handle": "alice",
  "verified": true,
  "message": "found"
}
```

Example: Using the adapter from application code

```rust
// in your integration test or production wiring
let client = sc_manager_adapters::rsi::SimpleRsiClient::new(vec!["alice"]);
let verifier = |h: &str| client.verify_handle(h);
let mut member_repo = sc_manager_app::in_memory_member_repo::InMemoryMemberRepo::new();
let mut handler = sc_manager_app::handlers::member_handler::MemberHandler::new(&mut member_repo);
let cmd = sc_manager_app::commands::AddMemberCommand::new("m1", Some("alice".to_string()), None);
let res = handler.add_with_rsi(cmd, verifier);
match res {
    Ok(()) => println!("Member added"),
    Err(e) => eprintln!("Failed to add member: {:?}", e),
}
```

Tip: Implement production adapter to return structured errors (HTTP status, JSON body) and map them to `Err(String)` with meaningful messages.