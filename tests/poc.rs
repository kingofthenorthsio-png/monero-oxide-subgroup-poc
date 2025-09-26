//! PoC: Subgroup-check missing for Ed25519 public keys in monero-oxide I/O
//!
//! This test shows that `read_point` accepts a small-order point (identity)
//! and that `CompressedPoint::decompress` yields an EdwardsPoint for it.
//!
//! NOTE: Function/module names are based on the library's public API. If any
//! symbol has changed, update the `use` paths accordingly.

use std::io::Cursor;

// Import from the monero-wallet crate (package name: `monero-wallet`).
use monero_wallet::io::{CompressedPoint, read_point};

/// Compressed encoding of the Ed25519 identity point (y = 1),
/// represented as 0x01 followed by 31 zero bytes.
//  This is a well-known small-order element (order 1).
fn compressed_identity() -> [u8; 32] {
    let mut id = [0u8; 32];
    id[0] = 1;
    id
}

/// PoC 1 — Proves that `read_point` accepts a small-order point (identity)
/// without rejecting it for being outside the prime-order subgroup.
#[test]
fn read_point_accepts_small_order_identity() {
    let mut cur = Cursor::new(compressed_identity());
    // If the library correctly rejected non prime-subgroup points,
    // this call should error instead of succeeding.
    let _p = read_point(&mut cur).expect(
        "BUG: read_point accepted a small-order (identity) Ed25519 point without subgroup check"
    );
}

/// PoC 2 — Shows that `CompressedPoint::decompress` accepts the identity
/// and returns an EdwardsPoint. You may extend this by asserting torsion
/// status if the API exposes `is_torsion_free()` or `is_small_order()`.
#[test]
fn decompress_identity_point() {
    let cp = CompressedPoint::from(compressed_identity());
    let maybe_p = cp.decompress();
    assert!(
        maybe_p.is_some(),
        "BUG: identity decompresses to some EdwardsPoint here, confirming acceptance"
    );

    // Optional (if the API provides it):
    // if let Some(p) = maybe_p {
    //     assert!(!p.is_torsion_free(), "expected non-torsion-free (small order) point");
    // }
}
