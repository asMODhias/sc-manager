//! Discord adapter (scoped, no business logic)

use serde::{Deserialize, Serialize};

/// Command payload received from Discord (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlashCommand {
    pub name: String,
    pub user_id: String,
    pub guild_id: Option<String>,
    pub options: Option<serde_json::Value>,
}

/// Adapter interface — keep business logic out of the adapter
pub trait DiscordAdapter: Send + Sync {
    fn handle_command(&self, cmd: SlashCommand) -> Result<String, String>;
}

/// Minimal in-process adapter used for tests and examples
pub struct InMemoryDiscordAdapter;

impl DiscordAdapter for InMemoryDiscordAdapter {
    fn handle_command(&self, cmd: SlashCommand) -> Result<String, String> {
        // Adapter is intentionally dumb — it forwards inputs as stringified output
        Ok(format!("handled:{}:{}", cmd.name, cmd.user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_handles_simple_command() {
        let adapter = InMemoryDiscordAdapter;
        let cmd = SlashCommand {
            name: "ping".into(),
            user_id: "user123".into(),
            guild_id: Some("guild456".into()),
            options: None,
        };

        let res = adapter.handle_command(cmd).unwrap();
        assert_eq!(res, "handled:ping:user123");
    }
}
