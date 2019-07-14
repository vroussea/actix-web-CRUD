use super::schema::users::dsl::*;
use crate::models;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyUser {
    pub id: String,
    pub name: String,
}

pub fn query(
    nm: String,
    pool: web::Data<Pool>,
) -> Result<Vec<models::User>, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    /*let filter = users
    .find(nm)
    .or(uuid);*/

    //let mut items = users.filter(filter).load::<models::User>(conn)?;
    let items = users.filter(name.eq(&nm)).load::<models::User>(conn)?;
    Ok(items)
}

pub fn query_all(pool: web::Data<Pool>) -> Result<Vec<models::User>, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let items = users.load::<models::User>(conn)?;
    Ok(items)
}

pub fn create(nm: String, pool: web::Data<Pool>) -> Result<models::User, diesel::result::Error> {
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

pub fn update(
    nm: String,
    uuid: String,
    pool: web::Data<Pool>,
) -> Result<models::User, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let clone_uuid = uuid.clone();

    diesel::update(users.filter(id.eq(clone_uuid)))
        .set(name.eq(nm))
        .execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

pub fn delete(uuid: String, pool: web::Data<Pool>) -> Result<String, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    diesel::delete(users.filter(id.eq(uuid.clone()))).execute(conn)?;

    Ok(uuid)
}
