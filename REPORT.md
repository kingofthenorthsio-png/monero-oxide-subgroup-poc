# High – Missing prime-subgroup check in monero-oxide (Ed25519) allows small-order tx pubkeys in scan pipeline

## Summary
`monero-oxide` accepte des points Ed25519 **hors sous-groupe premier** (torsion) :
- `io::read_point` décompresse une clé publique **sans** vérifier l’appartenance au sous-groupe premier ;
- `extra::Extra::read` accepte un `tx_pubkey` de **petit ordre** et l’expose via `Extra::keys()`.

## Impact (Severity = High)
Catégorie **Incorrect/Incomplete cryptographic formula**. Accepter des éléments de torsion peut produire des dérivations dégénérées et des résultats de scan incohérents (précondition à des faux reçus / omissions). Aucun accès mainnet.

## Affected components
- `monero_wallet::io::read_point` (après `CompressedPoint::decompress`)
- `monero_wallet::extra::Extra` (lecture d’un champ `tx_pubkey`)

## Proof of Concept
Tests dans ce dépôt (CI verte) :
- `tests/poc.rs::read_point_accepts_small_order_identity`
- `tests/poc_scan.rs::extra_accepts_small_order_tx_pubkey`

### Reproduction
