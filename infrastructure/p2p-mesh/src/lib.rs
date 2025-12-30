//! sc_manager_p2p_mesh: Basic CRDT state manager (initial in-memory implementation)

pub mod crdt;

pub use crdt::{CRDTStateManager, OrgState, CRDTError};

pub mod gossip;
pub use gossip::{HashGossipNode, HashGossipMessage, GossipUpdate};

pub mod mini_master;
pub use mini_master::{MiniMaster, MasterClient};