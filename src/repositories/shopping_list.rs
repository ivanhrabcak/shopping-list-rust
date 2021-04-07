#![allow(proc_macro_derive_resolution_fallback)]

use diesel::prelude::*;
use diesel::dsl::*;

use crate::models::{self, ShoppingList};

use crate::repositories::schema::shopping_lists;
use crate::repositories::schema::shopping_lists::dsl::*;

use std::convert::TryFrom;

pub fn create_shopping_list(list_title: String, conn: &PgConnection) -> Result<ShoppingList, ()> {
    let new_shopping_list_id: i64 = match shopping_lists::table.select(count(id)).first(conn) {
        Ok(shopping_list_id) => shopping_list_id,
        Err(_) => return Err(())
    };

    let new_shopping_list_id: i32 = match i32::try_from(new_shopping_list_id) {
        Ok(new_shopping_list_id) => new_shopping_list_id,
        Err(_) => return Err(())
    };

    let new_shopping_list = models::ShoppingList {
        id: new_shopping_list_id,
        title: list_title.to_string()
    };

    match diesel::insert_into(shopping_lists).values(&new_shopping_list).get_result::<(i32, String)>(conn) {
        Ok((new_id, new_title)) => Ok(ShoppingList { id: new_id, title: new_title }),
        Err(_) => Err(())
    }
}


pub fn remove_shopping_list(list_id: i32, conn: &PgConnection) -> Result<(), ()> {
    match delete(shopping_lists.filter(id.eq(list_id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn get_shopping_list_by_title(list_title: String, conn: &PgConnection) -> Result<ShoppingList, ()> {
    match shopping_lists.filter(title.eq(list_title)).get_result::<models::ShoppingList>(conn) {
        Ok(l) => Ok(l),
        Err(_) => Err(())
    }
}

pub fn get_shopping_list_by_id(list_id: i32, conn: &PgConnection) -> Result<ShoppingList, ()> {
    match shopping_lists.filter(id.eq(list_id)).get_result::<models::ShoppingList>(conn) {
        Ok(l) => Ok(l),
        Err(_) => Err(())
    }
}
