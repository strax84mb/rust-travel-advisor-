pub mod auth {
    //use actix_web::{FromRequest, Error};
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        iat: usize,
        exp: usize,
        roles: Vec<String>,
    }

    //pub struct AdminAuthorized;
/* 
    impl FromRequest for AdminAuthorized {
        type Error = Error;
        type Future = actix_web::Result<Self, Error>;

        fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
            Ok(AdminAuthorized)
        }
    }*/
}