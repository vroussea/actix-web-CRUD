extern crate reqwest;

use reqwest::StatusCode;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

pub fn get_user_repositories(
    username: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut res = client
        .get("https://api.github.com/user/repos")
        .header(
            "Authorization",
            "Basic ".to_owned() + &base64::encode(&(username + ":" + &password)),
        )
        .send()?;
    if res.status() == StatusCode::OK {
        Ok(res.text()?)
    } else {
        Err(Box::new(MyError("Oops".into())))
    }
}
