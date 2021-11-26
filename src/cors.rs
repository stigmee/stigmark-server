use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header};
use rocket::{Request, Response};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        println!("Fairing::on_response");
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        println!("Fairing::on_response");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
