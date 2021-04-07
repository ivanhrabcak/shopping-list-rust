use rocket::Route;
use rocket::{State};

use crate::controllers::user_controller::*;
use crate::controllers::shopping_list_controller::*;
use crate::repositories::connection::*;



#[get("/health")]
fn health(_conn: DbConn, _cache: State<Cache>) -> &'static str {
    return "Ok!";
}

pub fn get_routes() -> Vec<Route> {
    routes![health, 
            login, register, logout, 
            shopping_lists, create_list, remove_list, add_user_to_list, get_items, add_item, remove_item, set_item_done]
}