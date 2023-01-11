use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::HttpRequest;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Algorithm, Validation };

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    roles: Vec<String>,
}

use crate::util::app_errors::Error;

pub fn create_jwt(user: String, roles: Vec<String>) -> Result<String, Error> {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_millis() as usize,
        Err(err) => return Err(Error::underlying(err.to_string())),
    };

    let claims = Claims{
        sub: user.clone(),
        iat: now.clone(),
        exp: now + (3600 * 1000),
        roles: roles.clone(),
    };

    let headers = Header::new(Algorithm::RS256);
    let key = match EncodingKey::from_rsa_pem("".as_bytes()) {
        Ok(b) => b,
        Err(_err) => panic!("this should never happen"),
    };

    match encode(&headers, &claims, &key) {
        Ok(jwt) => Ok(jwt),
        Err(err) => Err(Error::underlying(err.to_string())),
    }
}

pub fn has_role(jwt: String, roles: Vec<String>) -> Result<bool, Error> {
    let key = DecodingKey::from_rsa_pem("".as_bytes()).unwrap();
    let claims = match decode::<Claims>(jwt.as_str(), &key, &Validation::default()) {
        Ok(c) => c.claims,
        Err(err) => return Err(Error::underlying(err.to_string())),
    };

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_millis() as usize,
        Err(err) => return Err(Error::underlying(err.to_string())),
    };

    if now > claims.exp {
        return Err(Error::underlying("token expired".to_string()));
    }

    if now < claims.iat {
        return Err(Error::underlying("token not valid yet".to_string()));
    }

    Ok(roles.iter().any(|r| claims.roles.contains(r)))
}

pub fn validate_request(req: &HttpRequest, roles: Vec<String>) -> Result<(), Error> {
    let mut authorization_value = match req.headers().get(actix_web::http::header::AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Ok(s) => s,
            Err(_err) => return Err(Error::underlying("Authorization header is not a string".to_string())),
        },
        None => return Err(Error::underlying("no Authorization header found".to_string())),
    };
    if !authorization_value.starts_with("Bearer ") {
        return Err(Error::underlying("Authorization header is not a JWT token".to_string()))
    }
    authorization_value = &authorization_value[6..];

    match has_role(authorization_value.to_string(), roles) {
        Ok(role_found) => {
            if !role_found {
                return Err(Error::underlying("operation not allowed for user".to_string()))
            }
        },
        Err(err) => return Err(Error::wrap(err, "".to_string())),
    };

    Ok(())
}
