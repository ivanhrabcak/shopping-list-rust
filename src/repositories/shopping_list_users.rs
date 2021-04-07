use diesel::prelude::*;
use diesel::dsl::*;

use crate::{models::{ShoppingListUser}, repositories::schema::shopping_list_users};
use crate::repositories::schema::shopping_list_users::dsl::*;

use std::convert::TryFrom;

pub fn assign_user_to_shopping_list(list_id: i32, u_id: i32, conn: &PgConnection) -> Result<ShoppingListUser, ()> {
    let assignment_exists = match shopping_list_users
                                        .filter(shopping_list_id.eq(list_id))
                                        .filter(user_id.eq(u_id))
                                        .get_result::<ShoppingListUser>(conn) {
        Ok(_) => true,
        Err(_) => false
    };

    if assignment_exists { 
        return Err(());
    }

    let new_id: i64 = match shopping_list_users::table.select(count(id)).first(conn) {
        Ok(new_id) => new_id,
        Err(_) => return Err(())
    };

    let new_id: i32 = match i32::try_from(new_id) {
        Ok(new_id) => new_id,
        Err(_) => return Err(())
    };


    let new_entry = ShoppingListUser {
        id: new_id,
        shopping_list_id: list_id,
        user_id: u_id
    };

    match insert_into(shopping_list_users).values(&new_entry).get_result::<ShoppingListUser>(conn) {
        Ok(shopping_list_user) => Ok(shopping_list_user),
        Err(_) => Err(())
    }
}

pub fn remove_assignment(list_id: i32, u_id: i32, conn: &PgConnection) -> Result<(), ()> {
    match delete(shopping_list_users)
            .filter(shopping_list_id.eq(list_id))
            .filter(user_id.eq(u_id))
            .get_result::<ShoppingListUser>(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn get_assignments_for_user_id(u_id: i32, conn: &PgConnection) -> Result<Vec<ShoppingListUser>, ()> {
    match shopping_list_users.filter(user_id.eq(u_id)).get_results(conn) {
        Ok(assignments) => Ok(assignments),
        Err(_) => Err(())
    }
}

pub fn get_assignments_for_shopping_list_id(list_id: i32, conn: &PgConnection) -> Result<Vec<ShoppingListUser>, ()> {
    match shopping_list_users.filter(shopping_list_id.eq(list_id)).get_results(conn) {
        Ok(assignments) => Ok(assignments),
        Err(_) => Err(())
    }
}