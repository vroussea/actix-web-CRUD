use super::schema::users::dsl::*;
use crate::models;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error::AlreadyInTransaction;
use crate::models::User;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyUser {
    pub id: String,
    pub name: String,
    pub github_name: String,
    pub github_password: String,
    pub password: String,
}

pub fn query(
    nm: String,
    pool: web::Data<Pool>,
) -> Result<Vec<models::User>, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let items = users.filter(name.eq(&nm)).load::<models::User>(conn)?;
    let mut corrected_items: Vec<User> = vec![];
    for mut item in items {
        item.password = String::from("*****");
        item.github_password = String::from("*****");
        corrected_items.push(item);
    }
    Ok(corrected_items)
}

pub fn query_all(pool: web::Data<Pool>) -> Result<Vec<models::User>, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let items = users.load::<models::User>(conn)?;
    let mut corrected_items: Vec<User> = vec![];
    for mut item in items {
        item.password = String::from("*****");
        item.github_password = String::from("*****");
        corrected_items.push(item);
    }
    Ok(corrected_items)
}

pub fn create(nm: String, github_nm: String, github_pwd: String, pwd: String, pool: web::Data<Pool>) -> Result<models::User, diesel::result::Error> {
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: nm.as_str(),
        github_name: github_nm.as_str(),
        github_password: github_pwd.as_str(),
        password: pwd.as_str(),
    };
    let conn: &SqliteConnection = &pool.get().unwrap();

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

pub fn update(
    nm: String, github_nm: String, github_pwd: String, pwd: String,
    uuid: String,
    pool: web::Data<Pool>,
) -> Result<models::User, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let clone_uuid = uuid.clone();

    let item = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;

    if item.len() == 1 && item.get(0).unwrap().password != pwd {
        return Err(AlreadyInTransaction);
    }

    diesel::update(users.filter(id.eq(clone_uuid)))
        .set((name.eq(nm), github_name.eq(github_nm), github_password.eq(github_pwd), password.eq(pwd)))
        .execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

pub fn delete(pwd: String, uuid: String, pool: web::Data<Pool>) -> Result<String, diesel::result::Error> {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let clone_uuid = uuid.clone();

    let item = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;

    if item.len() == 1 && item.get(0).unwrap().password != pwd {
        return Err(AlreadyInTransaction);
    }
    diesel::delete(users.filter(id.eq(clone_uuid))).execute(conn)?;

    Ok(uuid)
}
