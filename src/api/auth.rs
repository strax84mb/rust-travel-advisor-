use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Algorithm, Validation };

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    role: String,
}

mod errors {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub struct AuthError {
        pub cause_err: Box<dyn Error>,
    }

    impl Display for AuthError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "authentication failed: {}", self.cause_err.to_string())
        }
    }

    impl Error for AuthError {
        fn cause(&self) -> Option<&dyn Error> {
            Some(self.cause_err.as_ref())
        }
    }

    use std::time::SystemTimeError;

    impl AuthError {
        pub fn from(err: SimpleError) -> AuthError {
            AuthError { cause_err: Box::new(err) }
        }

        pub fn from_system_time_error(err: SystemTimeError) -> AuthError {
            AuthError { cause_err: Box::new(err) }
        }

        pub fn from_jsonwebtoken_error(err: jsonwebtoken::errors::Error) -> AuthError {
            AuthError { cause_err: Box::new(err) }
        }
    }

    #[derive(Debug)]
    pub struct SimpleError {
        msg: String,
    }

    impl Display for SimpleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.msg.clone())
        }
    }

    impl Error for SimpleError {
        fn cause(&self) -> Option<&dyn Error> {
            None
        }
    }

    impl SimpleError {
        pub fn new(msg: String) -> SimpleError {
            SimpleError { msg: msg }
        }
    }

}

use errors::{AuthError, SimpleError};

pub fn create_jwt(user: String, role: String) -> Result<String, AuthError> {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_millis() as usize,
        Err(err) => return Err(AuthError::from_system_time_error(err)),
    };

    let claims = Claims{
        sub: user.clone(),
        iat: now.clone(),
        exp: now + (3600 * 1000),
        role: role.clone(),
    };

    let headers = Header::new(Algorithm::RS256);
    let key = match EncodingKey::from_rsa_pem("".as_bytes()) {
        Ok(b) => b,
        Err(_err) => panic!("this should never happen"),
    };

    match encode(&headers, &claims, &key) {
        Ok(jwt) => Ok(jwt),
        Err(err) => Err(AuthError::from_jsonwebtoken_error(err)),
    }
}

pub fn has_role(jwt: String, roles: Vec<String>) -> Result<bool, AuthError> {
    let key = DecodingKey::from_rsa_pem("".as_bytes()).unwrap();
    let claims = match decode::<Claims>(jwt.as_str(), &key, &Validation::default()) {
        Ok(c) => c.claims,
        Err(err) => return Err(AuthError::from_jsonwebtoken_error(err)),
    };

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_millis() as usize,
        Err(err) => return Err(AuthError::from_system_time_error(err)),
    };

    if now > claims.exp {
        return Err(AuthError::from(SimpleError::new("token expired".to_string())));
    }

    if now < claims.iat {
        return Err(AuthError::from(SimpleError::new("token not valid yet".to_string())));
    }

    Ok(roles.iter().any(|r| r.eq(&claims.role)))
}