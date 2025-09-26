# Monero-Oxide Subgroup-Check PoC

This repository demonstrates an **incomplete cryptographic validation** in the `monero-oxide` wallet I/O layer: a public key (Ed25519) can be accepted **without verifying membership in the prime-order subgroup** (cofactor not cleared).

## What this PoC proves

- `read_point` accepts a **small-order** point (identity) without rejection.
- `CompressedPoint::decompress` accepts the identity and returns an `EdwardsPoint`.
- This is sufficient to claim **Incorrect/Incomplete cryptographic formula** (Immunefi, category = High).  
  If extended to a practical **false positive/negative** in output-scanning, it could qualify for **Critical** (“Reportedly received funds which weren’t actually received”).

> This PoC runs **locally** only. Do **not** interact with mainnet — follow Immunefi rules.

## How to run

```bash
# Install Rust if you haven't
# curl https://sh.rustup.rs -sSf | sh

# Run tests
cargo test -q
```

If the tests pass, the library accepted the small‑order point, which is the core of the issue.

## Suggested remediation

Inside the library's `read_point` (I/O module), after decompressing `CompressedPoint`, **reject torsion points**:

```rust
let p = cp.decompress().ok_or(Error::InvalidPoint)?;
// Reject small-order / non-torsion-free points
if !p.is_torsion_free() {
    return Err(Error::InvalidPoint);
}
```

(Depending on the `curve25519-dalek` API in use, you may use `is_small_order()` or `is_torsion_free()`.)

## Immunefi report tips (short)

- **Title:** Missing prime-subgroup check for Ed25519 points in `read_point` enables acceptance of small‑order keys
- **Impact:** Incorrect/incomplete cryptographic formula (High). Optional extension to Critical if a false-receipt scan is demonstrated.
- **PoC:** Include the output of `cargo test -q` and this repo.
- **Scope:** monero-oxide (wallet lib). No on-chain interaction; local only.
- **Remediation:** Add prime-subgroup membership check or cofactor clearing prior to use.
- **Disclosure:** Follows Immunefi rules; no production endpoints used.

## License

MIT
