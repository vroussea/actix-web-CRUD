#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserStruct {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserStruct {
    pub name: String,
    pub github_name: String,
    pub github_password: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserStruct {
    pub id: String,
    pub name: String,
    pub github_name: String,
    pub github_password: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteUserStruct {
    pub id: String,
    pub password: String,
}