#![allow(proc_macro_derive_resolution_fallback)]

use diesel::prelude::*;
use diesel::dsl::*;

use crate::models;

use crate::repositories::schema::users;
use crate::repositories::schema::users::dsl::*;
use crate::request::Credentials;

use crypto::sha2::Sha512;
use crypto::digest::Digest;

use std::convert::TryFrom;

#[derive(Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String
}

fn user_exists(user: NewUser, conn: &PgConnection) -> bool {
    match users::table
                .filter(username.eq(user.username))
                .select(id)
                .first::<i32>(conn) {
        Ok(val) => {
            println!("User exists, with id {}", val);
            true
        },
        Err(_) => {
            println!("User doesnt exist!");
            false
        }
    }
}

fn sha512_hash_string(string: String) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(&string);
    hasher.result_str()
}

pub fn create_user(user: NewUser, conn: &PgConnection) -> Result<models::User, ()> {   
    if user.password.len() != 128 {
        println!("Wrong length!");
        return Err(());
    }

    let new_username = &user.username;
    let does_user_exist = user_exists(user.clone(), conn);

    if does_user_exist {
        return Err(());
    }

    let new_user_id: i64 = match users::table.select(count(id)).first(conn) {
        Ok(user_id) => user_id,
        Err(_) => return Err(())
    };

    let new_user_id: i32 = match i32::try_from(new_user_id) {
        Ok(user_id) => user_id,
        Err(_) => return Err(())
    };

    let new_user = models::User {
        id: new_user_id,
        username: new_username.to_string(),
        password: user.password
    };
    
    match insert_into(users).values(&new_user).get_result::<(i32, String, String)>(conn) {
        Ok(_) => Ok(new_user.clone()),
        Err(_) => Err(())
    }
}

pub fn remove_user(user: models::User, user_password: String, conn: &PgConnection) -> Result<(), ()>{
    if sha512_hash_string(user_password) != user.password {
        return Err(());
    }
    
    match delete(users.filter(id.eq(user.id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn get_user_by_username(name: String, conn: &PgConnection) -> Result<models::User, ()> {
    match users.filter(username.eq(name)).get_result::<models::User>(conn) {
        Ok(u) => Ok(u),
        Err(_) => Err(())
    }
}

pub fn get_user_by_user_id(user_id: i32, conn: &PgConnection) -> Result<models::User, ()> {
    match users.filter(id.eq(user_id)).get_result::<models::User>(conn) {
        Ok(u) => Ok(u),
        Err(_) => Err(())
    }
}

pub fn get_user_with_credentials(credentials: Credentials, conn: &PgConnection) -> Result<models::User, ()> {
    match users.filter(username.eq(credentials.username)).select(id).get_result::<i32>(conn) {
        Ok(user_id) => {
            let user: models::User = match users.find(user_id).first(conn) {
                Ok(user) => user,
                Err(_) => return Err(())
            };

            if user.password == credentials.password {
                return Ok(user);
            }
            Err(())
        },
        Err(_) => Err(()) 
    }
}

pub fn change_user_password(user: models::User, old_password: String, new_password: String, conn: &PgConnection) -> Result<(), ()> {
    if old_password != user.password {
        return Err(());
    }

    match update(users.filter(id.eq(user.id))).set(password.eq(new_password)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}