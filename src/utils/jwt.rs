extern crate dotenv;

// use dotenv::dotenv;
use std::env;

use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use chrono::{Utc,Duration};
use jsonwebtoken::{Header, EncodingKey, encode};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    email: String
}

pub fn encode_jwt(email: String) ->Result<String, StatusCode>{
    let now = Utc::now();
    let expire = Duration::minutes(120);
    let claims = Claims{ iat: now.timestamp() as usize, exp: (now+expire).timestamp() as usize, email};
    let secret = env::var("SECURITY_KEY").expect("SECURITY_KEY not found");


    return encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
}