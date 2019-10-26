use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation, TokenData};

use showman_core::error::{Error, Result};
use showman_db::models::User;
use showman_db::models::role::Role;
use showman_db::{schema, establish_connection};

use crate::session;
use crate::session::SessionStatus;
use crate::SIGN_KEY;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    /// Subject of the claims, i.e. user_id
    sub: u32,
    /// User displayed name.
    name: String,
    /// Role of the user.
    role: Role,
    /// Expiration date.
    exp: i64,
    /// Issued at.
    iat: i64,
    /// Issuer.
    iss: String,
    /// Token ID.
    jti: u32,
    /// Auxiliary data used to refresh the token.
    aux: String,
    /// Session status.
    #[serde(skip)]
    session_status: SessionStatus
}

impl Claims {
    /// Creates a new `Claims` structure using the given user.
    pub fn from_auth(auth: User, token_id: u32, aux: &str) -> Claims {
        let timestamp = Utc::now().timestamp();
        Claims {
            sub: auth.user_id,
            name: auth.name,
            role: auth.role,
            // (Long) Expiration: 1 day.
            // Note: short expiration is 5 minutes.
            exp: timestamp + 86400,
            iat: timestamp,
            iss: "localhost".to_owned(),
            jti: token_id,
            aux: aux.to_owned(),
            session_status: SessionStatus::Valid
        }
    }

    /// Parses an existent token into a `Claims` structure.
    pub fn from_token(token: &str, aux: &str, key: &[u8]) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.iss = Some("localhost".to_string());
        let mut token_data: TokenData<Claims> = decode(token, key, &validation)
            .map_err(|_| Error::Unauthorized())?;

        let status = session::status(token_data.claims.token_id())?;

        token_data.claims.session_status = status;

        if status == SessionStatus::Invalid { return Err(Error::Unauthorized().finish()); }
        if (status == SessionStatus::Expired || token_data.claims.chk_time() > Utc::now())
            && token_data.claims.aux != aux {
            return Err(Error::Unauthorized().finish());
        }

        Ok(token_data.claims)
    }

    pub fn update(self, timestamp: i64, token_id: u32) -> Claims {
        Claims {
            exp: timestamp + 86400,
            iat: timestamp,
            jti: token_id,
            ..self
        }
    }

    pub fn token(&self) -> Result<String> {
        let header = Header::new(Algorithm::HS512);
        let result = encode(&header, self, &SIGN_KEY);
        match result {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::InternalServerError().finish())
        }
    }

    /// Obtains the ID of the user.
    pub fn user_id(&self) -> u32 {
        self.sub
    }

    /// Obtains the token ID.
    pub fn token_id(&self) -> u32 {
        self.jti
    }

    /// Obtains the role of the user.
    pub fn role(&self) -> Role {
        self.role
    }

    /// Obtains the creation time of the `Claims` structure.
    pub fn issue_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.iat, 0)
    }

    /// Obtains the expiration time of the `Claims` structure.
    pub fn exp_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.exp, 0)
    }

    /// Obtains the check time of the `Claims` structure.
    ///
    /// The check time is, by definition, 300 seconds (or 5 minutes) after the issue time, and is
    /// needed by the server to check if the client needs a new token or not.
    pub fn chk_time(&self) -> DateTime<Utc> {
        Utc.timestamp(self.iat + 300, 0)
    }

    /// Obtains the issuer of the `Claims` structure (in this case, `localhost`).
    pub fn issuer(&self) -> &str {
        &self.iss[..]
    }

    /// Obtains the string describing the user.
    pub fn display(&self) -> &str { &self.name[..] }

    pub fn needs_update(&self) -> bool {
        self.session_status != SessionStatus::Valid
    }

    pub fn fetch_update(&mut self, aux: &str) -> Result<()> {
        match self.session_status {
            SessionStatus::Valid => return Ok(()),
            SessionStatus::NeedsUpdate => {},
            SessionStatus::Expired => if self.aux != aux {
                return Err(Error::Unauthorized().finish());
            },
            SessionStatus::Invalid => return Err(Error::Unauthorized().finish())
        }

        let old_session = session::fetch(self.token_id())?;
        let timestamp = Utc::now().timestamp();
        let token_id = session::create(self.user_id(), timestamp + 300)?;

        let connection = establish_connection()?;

        let user: User = {
            use schema::user::dsl;

            dsl::user.filter(dsl::user_id.eq(old_session.user_id))
                .first::<User>(&connection)
                .map_err(|_| Error::InternalServerError())?
        };

        self.sub = user.user_id;
        self.aux = aux.to_owned();
        self.session_status = SessionStatus::Valid;
        self.iat = timestamp;
        self.exp = timestamp + 86400;
        self.jti = token_id;
        self.role = user.role;
        self.name = user.name.clone();

        Ok(())

    }
}