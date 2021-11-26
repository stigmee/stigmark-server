use rocket::request::{FromRequest, Outcome};
use rocket::{Request};
use rocket::http::Status;
use base64;

pub struct BasicAuth {
    pub name: String,
    pub pass: String,
}

impl BasicAuth {
    /// Creates a new [BasicAuth] struct/request guard from a given plaintext
    /// http auth header or returns a [Option::None] if invalid
    pub fn new<T: Into<String>>(auth_header: T) -> Option<Self> {
        let key = auth_header.into();
        if key.len() < 7 || &key[..6] != "Basic " {
            return None;
        }

        let decoded = match base64::decode(&key[6..]) {
            Ok(bytes) => String::from_utf8(bytes).unwrap(),
            Err(err) => format!("failed to decode {}", err),
        };

        let (name, pass) = decoded.split_once(":").unwrap();

        Some(Self {
            name: String::from(name),
            pass: String::from(pass),
        })
    }
}

#[derive(Debug)]
pub enum LoginError {
    InvalidUserPassCombination,
    BadCount,
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("BasicAuth::FromRequest keys={}", keys.join(","));
        match keys.len() {
            0 => Outcome::Success(BasicAuth{name: String::from(""), pass: String::from("")}),
            1 => match BasicAuth::new(keys[0]) {
                Some(auth_header) => Outcome::Success(auth_header),
                None => Outcome::Failure((Status::BadRequest, LoginError::InvalidUserPassCombination)),
            },
            _ => Outcome::Failure((Status::BadRequest, LoginError::BadCount)),
        }
    }
}
