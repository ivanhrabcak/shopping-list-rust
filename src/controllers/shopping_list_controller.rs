
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{
        models::{
            ShoppingList, 
            ShoppingListItem, 
            User
        }, 
        repositories::{
            connection::DbConn, 
            shopping_list::{
                create_shopping_list, 
                remove_shopping_list
            }, 
            shopping_list_items::{
                get_items_by_shopping_list_id
            }, 
            shopping_list_users::{
                assign_user_to_shopping_list, 
                get_assignments_for_shopping_list_id, 
                get_assignments_for_user_id, 
                remove_assignment
            }
        }, 
        request::{
            ListItem, 
            NewShoppingList, 
            UserId
        }, 
        utils::{
            does_user_have_permission_for_list
        }
    };
use crate::request::{Response};
use crate::repositories::shopping_list_items;

#[get("/lists")]
pub fn shopping_lists(user: User, conn: DbConn) -> Result<Response<Vec<i32>>, Status> {
    match get_assignments_for_user_id(user.id, &conn) {
        Ok(lists) => {
            let mut list_ids: Vec<i32> = Vec::new();
            for shopping_list_user in lists.iter() {
                list_ids.push(shopping_list_user.id);
            }


            Ok(Response { response: list_ids, status_code: Status::Ok.code })
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[put("/list", data = "<list>")]
pub fn create_list(_user: User, conn: DbConn, list: Json<NewShoppingList>) -> Result<Response<ShoppingList>, Status> {
    match create_shopping_list(list.name.clone(), &conn) {
        Ok(new_list) => Ok(Response { response: new_list, status_code: Status::Ok.code }),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/list/<id>")]
pub fn remove_list(user: User, conn: DbConn, id: i32) -> Result<Response<String>, Status> {
    let user_has_permission =  match does_user_have_permission_for_list(user, id, &conn) {
        Ok(p) => p,
        Err(_) => return Err(Status::InternalServerError)
    }; 

    if !user_has_permission {
        return Err(Status::Unauthorized)
    }

    match remove_shopping_list(id, &conn) {
        Ok(_) => (),
        Err(_) => return Err(Status::BadRequest)
    }

    let users_added_to_list = match get_assignments_for_shopping_list_id(id, &conn) {
        Ok(assignments) => assignments,
        Err(_) => return Ok(Response { response: "Success!".to_owned(), status_code: Status::Ok.code })
    };

    for u in users_added_to_list {
        match remove_assignment(u.shopping_list_id, u.user_id, &conn) {
            Ok(_) => (),
            Err(_) => return Err(Status::InternalServerError)
        };
    }
    
    
    Ok(Response { response: "Success!".to_owned(), status_code: Status::Ok.code })
}

#[post("/list/<id>/adduser", data = "<user_id>")]
pub fn add_user_to_list(_user: User, conn: DbConn, id: i32, user_id: Json<UserId>) -> Response<String> {
    match assign_user_to_shopping_list(id, user_id.id, &conn) {
        Ok(_) => Response { response: "Success!".to_owned(), status_code: Status::Ok.code },
        Err(_) => Response { response: "Failed.".to_owned(), status_code: Status::Conflict.code }
    }
}

#[post("/list/<id>/removeuser", data = "<user_id>")]
pub fn remove_user_from_list(_user: User, conn: DbConn, id: i32, user_id: Json<UserId>) -> Response<String> {
    match remove_assignment(id, user_id.id, &conn) {
        Ok(_) => Response { response: "Removed!".to_owned(), status_code: Status::Ok.code },
        Err(_) => Response { response: "Failed.".to_owned(), status_code: Status::BadRequest.code }
    }
}

#[get("/list/<id>")]
pub fn get_items(user: User, conn: DbConn, id: i32) -> Result<Response<Vec<ShoppingListItem>>, Status> {
    let user_has_permission =  match does_user_have_permission_for_list(user, id, &conn) {
        Ok(p) => p,
        Err(_) => return Err(Status::InternalServerError)
    }; 

    if !user_has_permission {
        return Err(Status::Unauthorized)
    }

    match get_items_by_shopping_list_id(id, &conn) {
        Ok(items) => Ok(Response { response: items, status_code: Status::Ok.code }),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[put("/list/<id>", data = "<new_item>")]
pub fn add_item(user: User, conn: DbConn, id: i32, new_item: Json<ListItem>) -> Result<Response<ShoppingListItem>, Status> {
    let user_has_permission =  match does_user_have_permission_for_list(user, id, &conn) {
        Ok(p) => p,
        Err(_) => return Err(Status::InternalServerError)
    }; 

    if !user_has_permission {
        return Err(Status::Unauthorized)
    }

    match shopping_list_items::add_item(id, new_item.name.clone(), new_item.done, &conn) {
        Ok(new_item) => Ok(Response { response: new_item, status_code: Status::Ok.code }),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/list/<id>/<item_id>")]
pub fn remove_item(user: User, conn: DbConn, id: i32, item_id: i32) -> Result<Response<String>, Status> {
    let user_has_permission =  match does_user_have_permission_for_list(user, id, &conn) {
        Ok(p) => p,
        Err(_) => return Err(Status::InternalServerError)
    }; 

    if !user_has_permission {
        return Err(Status::Unauthorized)
    }

    match shopping_list_items::remove_item(item_id, &conn) {
        Ok(_) => Ok(Response { response: "Success!".to_owned(), status_code: Status::Ok.code }),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[post("/list/<id>/<item_id>?<done>")]
pub fn set_item_done(user: User, conn: DbConn, id: i32, item_id: i32, done: bool) -> Result<Response<String>, Status> {
    let user_has_permission =  match does_user_have_permission_for_list(user, id, &conn) {
        Ok(p) => p,
        Err(_) => return Err(Status::InternalServerError)
    }; 

    if !user_has_permission {
        return Err(Status::Unauthorized)
    }

    

    match shopping_list_items::set_done(item_id, done, &conn) {
        Ok(_) => Ok(Response { response: "Success!".to_owned(), status_code: Status::Ok.code }),
        Err(_) => Err(Status::InternalServerError)
    }
}

