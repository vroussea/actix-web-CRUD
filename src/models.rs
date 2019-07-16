use super::schema::users;

#[derive(Serialize, Queryable, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub github_name: String,
    pub github_password: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub github_name: &'a str,
    pub github_password: &'a str,
    pub password: &'a str,
}
