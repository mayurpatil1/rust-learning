// Integration test — lives in tests/, compiled as a SEPARATE crate.
// It can only use the PUBLIC API, exactly like a real user of the library.
// No access to private internals — that's the point.

use _docs::{most_valuable, total_value, Item};

#[test]
fn a_realistic_haul() {
    let haul = vec![
        Item::new("dagger", 45),
        Item::new("shield", 120),
        Item::new("amulet", 300),
    ];

    // Total value, via the public function.
    assert_eq!(total_value(&haul), 465);

    // Most valuable item, via the public function.
    let best = most_valuable(&haul).unwrap();
    assert_eq!(best.name, "amulet");
    assert_eq!(best.value, 300);
}

#[test]
fn empty_haul_behaves() {
    assert_eq!(total_value(&[]), 0);
    assert!(most_valuable(&[]).is_none());
}