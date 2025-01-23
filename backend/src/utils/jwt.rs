
use std::future;

use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::constants;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TokenData<T> {
//     pub header: Header,
//     pub claims: T,
// }

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claims")))
        }
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}

pub fn encode_jwt(email: String, id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::hours(5);

    let claims = Claims {
        exp: (now+expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id
    };
    

    let secret = (*constants::SECRET).clone();

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_ref())
    )
}

pub fn decode_token(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = (*constants::SECRET).clone();

    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &token, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );

    claim_data
}