use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use ttl_cache::TtlCache;

use crate::{models::User, repositories::{connection::DbConn, shopping_list_users::get_assignments_for_user_id}};

pub fn generate_token() -> String {
    let mut rng = thread_rng();
    let mut output = String::new();
    for i in 0..5 {
        let chars: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(4)
                .collect();
        output += &chars;

        if i != 4 {
            output += "-";
        }
    }

    output
}

pub fn is_token_valid(token: String, cache: TtlCache<i32, String>) -> bool {
    for (_k, v) in cache.clone().iter() {
        if v.to_owned().eq(&token) {
            return true;
        }
    }
    false
}


pub fn invalidate_token(token: String, cache: &mut TtlCache<i32, String>) -> Result<(), ()> {
    for (k, v) in cache.clone().iter() {
        if v.to_owned().eq(&token) {
            cache.remove(k);
            return Ok(());
        }
    }

    Err(())
}

pub fn does_user_have_permission_for_list(user: User, list_id: i32, conn: &DbConn) -> Result<bool, ()> {
    let user_shopping_lists = match get_assignments_for_user_id(user.id, &conn) {
        Ok(u) => u,
        Err(_) => return Err(())
    };

    let mut is_allowed_to_shopping_list = false;

    for shopping_list in user_shopping_lists {
        if shopping_list.shopping_list_id == list_id {
            is_allowed_to_shopping_list = true;
        }
    }

    Ok(is_allowed_to_shopping_list)    
}