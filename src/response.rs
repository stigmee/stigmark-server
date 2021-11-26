use rocket::{Request, Response};
use rocket::response::{Responder, NamedFile};
use rocket::http::Status;
use rocket::response;
use rocket_contrib::json::JsonValue;
use std::path::Path;

#[derive(Debug)]
pub enum ServerResponse {
    Ok(),
    File(NamedFile),
    Json(JsonValue),
    Error(Status),
    BasicAuth(),
}

impl ServerResponse {
    pub fn ok() -> Self {
        Self::Ok()
    }

    pub fn file(file: &Path) -> Self {
        match NamedFile::open(file) {
            Ok(file) => Self::File(file),
            Err(_) => Self::Error(Status::NotFound),
        }
    }

    #[allow(dead_code)]
    pub fn json(json: JsonValue) -> Self {
        Self::Json(json)
    }

    pub fn basic_auth() -> Self {
        Self::BasicAuth()
    }
}

impl<'r>  Responder<'r> for ServerResponse {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        let mut res = Response::build();
        match self {
            Self::Ok() => res.status(Status::Ok),
            Self::File(file) => res.sized_body(file),
            Self::Error(status) => res.status(status),
            Self::BasicAuth() => {
                res
                    .raw_header("WWW-Authenticate", "Basic realm=\"User Visible Realm\", charset=\"UTF-8\"")
                    .status(Status::Unauthorized)
            }
            _ => res.status(Status::NotImplemented),
        }.ok()
    }
}
