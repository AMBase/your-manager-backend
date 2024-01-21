use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use sha2::Sha256;

use crate::db::users::User;


pub fn jwt_encode(user: &User) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user.id);

    claims.sign_with_key(&key).unwrap()
}

pub fn jwt_decode(token: &String) -> BTreeMap<String, String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key).unwrap();
    
    token.claims().clone()
}
