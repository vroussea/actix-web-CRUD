use actix_web::{web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models;
use super::schema::users::dsl::*;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn query() {

}

pub fn query_all() {

}

pub fn create(nm: String,
              pool: web::Data<Pool>,
) -> Result<models::User, diesel::result::Error> {

    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: nm.as_str(),
    };
    let conn: &SqliteConnection = &pool.get().unwrap();

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

pub fn update() {

}

pub fn delete() {

}