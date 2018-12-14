use witnet_crypto::hash::{calculate_sha256, Sha256};

#[test]
fn sha256() {
    let expected_witnet_hash = [
        0xe2, 0xed, 0xcd, 0x3d, 0xc8, 0x06, 0x6b, 0x71, 0x5d, 0xfa, 0x78, 0x80, 0x34, 0x9c, 0x24,
        0xc3, 0xb7, 0x97, 0x5a, 0x29, 0xfe, 0x54, 0xfd, 0x26, 0x19, 0xe5, 0xe4, 0x95, 0x29, 0x55,
        0x5f, 0xe3,
    ];
    let witnet_hash = calculate_sha256(b"WITNET");
    assert_eq!(witnet_hash, Sha256(expected_witnet_hash));
}
