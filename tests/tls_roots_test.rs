//! rtk's only in-process HTTPS client is ureq 2 (telemetry ping and erasure),
//! which verifies TLS against the compiled-in `webpki-roots` list rather than
//! the OS trust store. These checks pin that list to a release without roots
//! that Mozilla has distrusted, so a lockfile downgrade fails CI.

/// Roots removed after their issued intermediates were found to have
/// recoverable private keys (eMudhra incident, 2026-10-17).
const DISTRUSTED_ROOTS: &[&str] = &["emSign ECC Root CA - G3"];

#[test]
fn compiled_in_roots_exclude_distrusted_emudhra_root() {
    for name in DISTRUSTED_ROOTS {
        let present = webpki_roots::TLS_SERVER_ROOTS.iter().any(|anchor| {
            let subject: &[u8] = anchor.subject.as_ref();
            subject
                .windows(name.len())
                .any(|window| window == name.as_bytes())
        });
        assert!(
            !present,
            "webpki-roots still trusts distrusted root {name:?}"
        );
    }
}

#[test]
fn compiled_in_roots_are_not_empty() {
    // Guards the check above against vacuously passing on an empty list.
    assert!(webpki_roots::TLS_SERVER_ROOTS.len() > 100);
}
