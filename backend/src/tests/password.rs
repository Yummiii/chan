use crate::utils::password::{hash_password, verify_password};

const PASSWORD: &str = "password";

#[test]
fn hasher() {
    let hash = hash_password(PASSWORD);
    assert_ne!(hash, PASSWORD);
    assert_eq!(hash.len(), 97);
    assert!(verify_password(PASSWORD, &hash));
}
