pub mod auth {
    use std::borrow::Borrow;

    static mut auth_service: AuthServiceImpl = AuthServiceImpl{
        temp: Option::None
    };

    pub trait AuthService {
        fn login(&self) -> String;
    }

    struct AuthServiceImpl {
        pub temp: Option<String>
    }

    impl AuthService for AuthServiceImpl {
        fn login(&self) -> String {
            self.temp.clone().unwrap()
        }
    }

    pub fn new_auth_service() -> &'static impl AuthService {
        unsafe {
            auth_service = AuthServiceImpl {
                temp: Option::Some("temp".to_string())
            };
            auth_service.borrow()
        }
    }

    pub fn get_auth_service() -> &'static impl AuthService {
        unsafe {
            auth_service.borrow()
        }
    }
}