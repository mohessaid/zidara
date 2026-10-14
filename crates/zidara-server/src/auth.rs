//! Demo sign-in tokens (HS256). OIDC replaces this after the prototype; the rest of the server
//! only ever sees "this request acts as user X".

use std::fmt;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zidara_kernel::ids::Timestamp;

pub const TOKEN_LIFETIME_MS: i64 = 12 * 60 * 60 * 1000;

#[derive(Clone)]
pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn from_secret(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl fmt::Debug for Keys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Keys(..)")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthError {
    #[error("the token is invalid")]
    Invalid,
    #[error("the token has expired")]
    Expired,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    /// Expiry in milliseconds since the epoch, checked against the server's clock.
    exp_ms: i64,
}

pub fn issue(keys: &Keys, user: Uuid, now: Timestamp) -> String {
    let claims = Claims {
        sub: user,
        exp_ms: now.0 + TOKEN_LIFETIME_MS,
    };
    encode(&Header::new(Algorithm::HS256), &claims, &keys.encoding)
        .expect("HS256 signing does not fail")
}

pub fn verify(keys: &Keys, token: &str, now: Timestamp) -> Result<Uuid, AuthError> {
    let mut validation = Validation::new(Algorithm::HS256);
    // Expiry is checked below against the injected clock, so tests control time.
    validation.validate_exp = false;
    validation.required_spec_claims.clear();
    let claims = decode::<Claims>(token, &keys.decoding, &validation)
        .map_err(|_| AuthError::Invalid)?
        .claims;
    if now.0 >= claims.exp_ms {
        return Err(AuthError::Expired);
    }
    Ok(claims.sub)
}
