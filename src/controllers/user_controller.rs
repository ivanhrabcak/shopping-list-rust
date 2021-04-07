use rocket::{State, http::Status};
use rocket_contrib::json::Json;

use ttl_cache::TtlCache;

use std::{sync::{Arc, Mutex}, time::Duration};

use crate::{models::User, utils::{generate_token, invalidate_token}};
use crate::repositories::user::{NewUser, create_user, get_user_with_credentials};
use crate::repositories::connection::*;
use crate::request::{Credentials, Response};


pub type Cache = Arc<Mutex<TtlCache<i32, String>>>;

#[post("/register", data = "<credentials>")]
pub fn register(credentials: Json<Credentials>, conn: DbConn) -> Json<Response<String>> {
    match create_user(NewUser { username: credentials.username.clone(), password: credentials.password.clone() }, &conn) {
        Ok(_) => Json(Response { response: "Account created".to_string(), status_code: Status::Ok.code }),
        Err(_) => Json(Response { 
            response: "An account with that username already exists, or the hash has a wrong length".to_string(),
            status_code: Status::Conflict.code
        }) 
    }
}

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>, token_cache: State<Cache>, conn: DbConn) -> Json<Response<String>> {
    let credentials: Credentials = Credentials { username: credentials.username.clone(), password: credentials.password.clone() };
    let user = match get_user_with_credentials(credentials, &conn) {
        Ok(user) => user,
        Err(_) => return Json(Response { response: "Failed to log in.".to_string(), status_code: Status::BadRequest.code })
    };

    let token = generate_token();

    token_cache.lock().unwrap().insert(user.id, token.clone(), Duration::from_secs(60 * 60 * 2));

    return Json(Response { response: token, status_code: Status::Ok.code });
}

#[post("/logout?<token>")]
pub fn logout(_user: User, token: String, token_cache: State<Cache>) -> Response<String> {
    let mut cache = token_cache.lock().unwrap();

    match invalidate_token(token, &mut cache) {
        Ok(_) => Response { response: "Ok".to_string(), status_code: Status::Ok.code },
        Err(_) => Response { response: "Bad token.".to_string(), status_code: Status::BadRequest.code }
    }
    
    
}