use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::db::users::User;

pub fn jwt_encode(user: &User) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user.id);

    claims.sign_with_key(&key).unwrap()
}