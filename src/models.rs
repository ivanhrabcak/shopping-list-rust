use diesel::{Queryable, Insertable};

use rocket::{Outcome, State, http::Status, request::FromRequest};
use serde::{Serialize, Deserialize};

use crate::{controllers::user_controller::Cache, repositories::{connection::DbConn, user::get_user_by_user_id}};

use crate::repositories::schema::shopping_list_items;
use crate::repositories::schema::shopping_list_users;
use crate::repositories::schema::shopping_lists;
use crate::repositories::schema::users;

#[derive(Queryable, Insertable, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let token = request.get_query_value::<String>("token");
        if token.is_none() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let token = match token.unwrap() {
            Ok(t) => t,
            Err(_) => return Outcome::Failure((Status::BadRequest, ()))
        };

        let token_cache = match request.guard::<State<Cache>>() {
            Outcome::Success(cache) => cache,
            _ => return Outcome::Failure((Status::InternalServerError, ()))
        };

        let mut user_id: i32 = -1;

        let token_cache = token_cache.lock().unwrap();
        for (k, v) in token_cache.clone().iter() {
            if v.eq(&token) {
                user_id = *k;
                break;
            }
        }

        if user_id == -1 {
            return Outcome::Failure((Status::Unauthorized, ()))
        }

        let db_conn = match request.guard::<DbConn>() {
            Outcome::Success(conn) => conn,
            _ => return Outcome::Failure((Status::InternalServerError, ())),
        };
        
        match get_user_by_user_id(user_id, &db_conn) {
            Ok(user) => Outcome::Success(user),
            Err(_) => Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}

#[derive(Queryable, Insertable, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[table_name = "shopping_list_users"]
pub struct ShoppingListUser {
    pub id: i32,
    pub shopping_list_id: i32,
    pub user_id: i32
}

#[derive(Queryable, Insertable, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[table_name = "shopping_list_items"]
pub struct ShoppingListItem {
    pub id: i32,
    pub shopping_list_id: i32,
    pub item_name: Option<String>,
    pub done: bool
}

#[derive(Queryable, Insertable, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[table_name = "shopping_lists"]
pub struct ShoppingList {
    pub id: i32,
    pub title: String
}
