use chrono::Utc;

use jsonwebtoken::{Header, Algorithm, encode, Validation, TokenData, decode};

use showman_core::error::{Error, Result};

use crate::SIGN_KEY;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    /// Subject of the claims, i.e. user_id
    sub: u32,
    /// Issued at.
    iat: i64
}

impl Claims {
    pub fn new(user_id: u32) -> Claims {
        let sub = user_id;
        let iat = Utc::now().timestamp();

        Claims {sub, iat}
    }

    pub fn to_token(&self) -> Result<String> {
        let header = Header::new(Algorithm::HS512);
        let result = encode(&header, self, &SIGN_KEY);
        match result {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::InternalServerError().finish())
        }
    }

    pub fn from_token(token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.validate_exp = false;

        let token_data: TokenData<Claims> = decode(token, &SIGN_KEY, &validation)
            .map_err(|_| Error::Unauthorized().finish())?;

        let iat = Utc::now().timestamp();

        if token_data.claims.iat >= iat {
            return Err(Error::Unauthorized().finish());
        }

        Ok(token_data.claims)
    }

    pub fn user_id(&self) -> u32 {
        self.sub
    }
}