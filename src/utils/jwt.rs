extern crate dotenv;

// use dotenv::dotenv;
use std::env;

use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use chrono::{Utc,Duration};
use jsonwebtoken::{Header, EncodingKey, encode, decode, DecodingKey, Validation, TokenData};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String
}

pub fn encode_jwt(email: String) ->Result<String, StatusCode>{
    let now = Utc::now();
    let expire = Duration::minutes(120);
    let claims = Claims{ iat: now.timestamp() as usize, exp: (now+expire).timestamp() as usize, email};
    let secret = env::var("SECURITY_KEY").expect("SECURITY_KEY not found");


    return encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
}

pub fn decode_jwt(jwt: String) ->  Result<TokenData<Claims>,StatusCode>{
    let secret = env::var("SECURITY_KEY").expect("SECURITY_KEY not found");
    let res: Result<TokenData<Claims>, StatusCode> = decode::<Claims>(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).map_err(|_| { StatusCode::UNAUTHORIZED });

    return res;
}