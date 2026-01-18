//! Portal definitions and validation helpers (C-TB-4)
//!
//! Defines the seeded portals required by SR-DIRECTIVE and SR-CONTRACT and
//! exposes a small helper for whitelist enforcement.

/// Seeded portal identifiers per SR-DIRECTIVE §1.1
pub const SEEDED_PORTALS: &[&str] = &[
    "HumanAuthorityExceptionProcess",
    "GovernanceChangePortal",
    "ReleaseApprovalPortal",
];

/// Returns true if the provided portal_id matches a seeded portal
pub fn is_seeded_portal(portal_id: &str) -> bool {
    SEEDED_PORTALS.contains(&portal_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_portals_are_whitelisted() {
        for portal in SEEDED_PORTALS {
            assert!(is_seeded_portal(portal));
        }
    }

    #[test]
    fn non_seeded_portals_are_rejected() {
        assert!(!is_seeded_portal("UnapprovedPortal"));
        assert!(!is_seeded_portal("ReleasePortal")); // common alias should still fail
    }
}
