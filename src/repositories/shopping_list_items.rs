use diesel::prelude::*;
use diesel::dsl::*;

use crate::{models::{ShoppingListItem}, repositories::schema::shopping_list_items};
use crate::repositories::schema::shopping_list_items::dsl::*;

use std::convert::TryFrom;


pub fn add_item(list_id: i32, name: Option<String>, is_done: bool, conn: &PgConnection) -> Result<ShoppingListItem, ()> {
    let new_item_id: i64 = match shopping_list_items::table.select(count(id)).first(conn) {
        Ok(new_id) => new_id,
        Err(_) => return Err(())
    };

    let new_item_id: i32 = match i32::try_from(new_item_id) {
        Ok(new_id) => new_id,
        Err(_) => return Err(())
    };

    let new_item = ShoppingListItem {
        id: new_item_id,
        shopping_list_id: list_id,
        item_name: name,
        done: is_done
    };

    match insert_into(shopping_list_items).values(&new_item).get_result::<ShoppingListItem>(conn) {
        Ok(new_item) => Ok(new_item),
        Err(_) => Err(())
    }
}

pub fn set_done(item_id: i32, is_done: bool, conn: &PgConnection) -> Result<(), ()> {
    match update(shopping_list_items.filter(id.eq(item_id))).set(done.eq(is_done)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn set_name(item_id: i32, new_name: String, conn: &PgConnection) -> Result<(), ()> {
    match update(shopping_list_items.filter(id.eq(item_id))).set(item_name.eq(new_name)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn get_item_by_id(item_id: i32, conn: &PgConnection) -> Result<ShoppingListItem, ()> {
    match shopping_list_items.filter(id.eq(item_id)).get_result::<ShoppingListItem>(conn) {
        Ok(item) => Ok(item),
        Err(_) => Err(())
    }
}

pub fn get_items_by_shopping_list_id(list_id: i32, conn: &PgConnection) -> Result<Vec<ShoppingListItem>, ()> {
    match shopping_list_items.filter(shopping_list_id.eq(list_id)).get_results::<ShoppingListItem>(conn) {
        Ok(val) => Ok(val),
        Err(_) => Err(())
    }
}

pub fn remove_item(item_id: i32, conn: &PgConnection) -> Result<(), ()> {
    match delete(shopping_list_items)
            .filter(id.eq(item_id))
            .execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}