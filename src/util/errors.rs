pub mod errors_mod {
    use std::fmt::{Display, Debug};

    #[derive(Debug, PartialEq)]
    pub enum Reason {
        Wrapped,
        NotFound,
        Underlying,
    }

    #[derive(Debug)]
    pub struct Error {
        reason: Reason,
        msg: String,
        cause: Option<Box<Error>>
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let step_one = write!(f, "{}: ", self.msg.clone());
            let cause = self.cause.as_ref();
            match step_one {
                Ok(_) => {
                    if cause.is_some() {
                        let e =cause.as_deref().unwrap();
                        <Error as Display>::fmt(e.as_ref(), f)
                    } else {
                        Ok(())
                    }
                }
                Err(err) => return Err(err),
            }
        }
    }

    impl std::error::Error for Error {
        fn cause(&self) -> Option<&dyn std::error::Error> {
            None
        }
    }

    impl Error {
        pub fn not_found() -> Error {
            Error {
                reason: Reason::NotFound,
                msg: "not found".to_string(),
                cause: None,
            }
        }

        pub fn underlying(msg: String) -> Error {
            Error {
                reason: Reason::Underlying,
                msg: msg,
                cause: None,
            }
        }

        pub fn wrap(err: Error, msg: String) -> Error {
            Error {
                reason: Reason::Wrapped,
                msg: msg,
                cause: Some(Box::new(err)),
            }
        }

        pub fn type_message(&self, reason: Reason) -> Option<String> {
            let mut cause: Option<&Error> = Some(&self);
            loop {
                match cause {
                    Some(e) => {
                        if e.reason == reason {
                            return Some(e.msg.clone());
                        } else if e.cause.is_some() {
                            cause = e.cause.as_deref();
                        } else {
                            cause = None;
                        }
                    },
                    None => break,
                }
            };

            None
        }

        pub fn message(&self) -> String {
            self.msg.clone()
        }
    }

}
