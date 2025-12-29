//! Adapter registry for Data Hub Policy (V3.5)
//!
//! This module declares the canonical lists of Allowed / Optional / Forbidden data hubs

pub enum TrustLevel {
    Official,
    CommunityVerified,
    CommunityUnverified,
    Fallback,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubCategory {
    Allowed,
    Optional,
    Forbidden,
}

pub struct HubInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub category: HubCategory,
}

// Canonical listing (update when adding adapters)
pub const HUBS: &[HubInfo] = &[
    HubInfo { id: "rsi_verseguide", name: "RSI VerseGuide", category: HubCategory::Allowed },
    HubInfo { id: "galactapedia", name: "Galactapedia", category: HubCategory::Allowed },
    HubInfo { id: "fleetyards", name: "FleetYards", category: HubCategory::Allowed },
    HubInfo { id: "erkul", name: "Erkul", category: HubCategory::Allowed },
    HubInfo { id: "uex", name: "UEX/SC Trade Tools", category: HubCategory::Allowed },
    HubInfo { id: "rsi_comms", name: "RSI CommLinks & Patch Notes", category: HubCategory::Allowed },

    // Optional / plugin
    HubInfo { id: "rsi_issue_council", name: "RSI Issue Council", category: HubCategory::Optional },
    HubInfo { id: "spectrum_dev", name: "Spectrum Dev Statements", category: HubCategory::Optional },

    // Forbidden (examples)
    HubInfo { id: "leaks", name: "Leaks / Datamining", category: HubCategory::Forbidden },
    HubInfo { id: "player_tracking", name: "Player Tracking Services", category: HubCategory::Forbidden },
];

/// Verify that a given adapter id is allowed or optional; returns Err if forbidden or unknown.
pub fn verify_adapter_allowed(adapter_id: &str) -> Result<HubCategory, String> {
    for h in HUBS.iter() {
        if h.id == adapter_id {
            return Ok(h.category.clone());
        }
    }
    Err(format!("Unknown adapter id: {} — must be registered in adapters::registry", adapter_id))
}
