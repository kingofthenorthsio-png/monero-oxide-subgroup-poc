use std::io::Cursor;

// API déjà utilisée dans ton premier test : même module.
use monero_wallet::io::{CompressedPoint};

// Ces imports existent dans la lib monero-oxide; si l'éditeur
// te souligne en rouge, garde ce fichier et dis-moi — je t’enverrai
// la variante d’import (les noms peuvent changer légèrement).
use monero_wallet::extra::Extra;

/// Encodage compressé de l'identité Ed25519 (y=1).
fn compressed_identity() -> [u8; 32] {
    let mut id = [0u8; 32];
    id[0] = 1;
    id
}

/// PoC scan — prouve qu'un `tx_pubkey` de **petit ordre**
/// dans `extra` est **accepté** et exposé par l'API `Extra`.
#[test]
fn extra_accepts_small_order_tx_pubkey() {
    // 1) Construire un "extra" valide minimal: tag TX_PUBLIC_KEY (=0x01) + 32 octets
    let mut bytes = Vec::with_capacity(1 + 32);
    bytes.push(0x01); // tag 'tx public key'
    bytes.extend_from_slice(&compressed_identity());

    // 2) Parser via la lib
    let mut cur = Cursor::new(bytes);
    let extra = Extra::read(&mut cur).expect("Extra::read should parse our tx pubkey field");

    // 3) Vérifier que la lib **expose** la clé (malgré petit ordre)
    let maybe_keys = extra.keys();
    assert!(maybe_keys.is_some(), "keys() should be Some for a tx pubkey");
    let (tx_keys, additional) = maybe_keys.unwrap();

    assert_eq!(tx_keys.len(), 1, "expected exactly one tx pubkey");
    assert!(additional.is_none() || additional.as_ref().unwrap().is_empty(), "no additional keys expected");

    // 4) Optionnel: montrer que l'identité se "décompresse" (comme dans le premier test)
    let cp = CompressedPoint::from(compressed_identity());
    assert!(cp.decompress().is_some(), "identity decompresses to an EdwardsPoint (accepted)");
}
