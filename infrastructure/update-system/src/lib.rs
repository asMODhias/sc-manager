//! Update system crate - minimal scaffold
//!
//! Provides modules for authoring updates, client verification, delta generation,
//! signatures and rollback helpers. This is a small, testable baseline to follow
//! the TASK-001 specification in the project SOT.

pub mod author;
pub mod client;
pub mod delta;
pub mod signature;
pub mod rollback;

pub use author::AuthorService;
pub use client::ClientService;
pub use delta::chunks as delta_chunks;
pub use delta::apply_chunks as delta_apply_chunks;
pub use signature::{generate_keypair, sign_with_keypair, verify_signature_ed25519};
pub use rollback::{create_backup, transactional_replace, restore_backup};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_smoke() {
        // Basic smoke test to ensure the crate can be compiled and used
        assert_eq!("ok", "ok");
    }
}
