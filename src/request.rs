use std::io::Cursor;

use rocket::{http::{ContentType, Status}, response::Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub response: T,
    pub status_code: u16
}

impl<'r, T> Responder<'r> for Response<T> where T: Serialize {
    fn respond_to(self, _request: &rocket::Request) -> rocket::response::Result<'r> {
        let json_string = match serde_json::to_string_pretty(&self) {
            Ok(result) => result,
            Err(_) => return Result::Err(Status::InternalServerError)
        };

        let response = rocket::Response::build()
            .sized_body(Cursor::new(json_string))
            .header(ContentType::new("application", "json"))
            .status(Status::from_code(self.status_code).unwrap())
            .finalize();

        
        Result::Ok(response)
    }
}

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String 
}


#[derive(Deserialize, Debug)]
pub struct NewShoppingList {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct UserId {
    pub id: i32
}

#[derive(Deserialize, Debug)]
pub struct ListItem {
    pub name: Option<String>,
    pub done: bool
}