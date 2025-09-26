# Missing prime-subgroup check in monero-oxide (Ed25519) allows small-order tx pubkeys to be accepted

## Summary
`monero-oxide` accepte des points Ed25519 **hors sous-groupe premier** (cofacteur non nettoyé) dans le pipeline d’E/S et de parsing `extra`.
- `io::read_point` décompresse une clé publique **sans** vérifier l’appartenance au sous-groupe premier.
- `extra::Extra::read` accepte un `tx_pubkey` de **petit ordre** et l’expose via `Extra::keys()`.

## Impact
**High – Incorrect/Incomplete cryptographic formula.**  
Accepter des éléments de torsion permet des dérivations dégénérées et des résultats de scan incohérents.  
(*Extension possible vers Critical si l’on démontre “Reportedly received funds which weren’t actually received”.*)

## PoC
Deux tests dans ce dépôt :
- `tests/poc.rs::read_point_accepts_small_order_identity`
- `tests/poc_scan.rs::extra_accepts_small_order_tx_pubkey`

Repro : `cargo test -q` (ou voir l’onglet **Actions** CI vert).

## Affected components
- `monero_wallet::io::read_point` (après `CompressedPoint::decompress`)
- `monero_wallet::extra::Extra` (lecture d’un champ `tx_pubkey`)

## Suggested remediation
Après décompression :
```rust
let p = cp.decompress().ok_or(Error::InvalidPoint)?;
if !p.is_torsion_free() { return Err(Error::InvalidPoint); }
