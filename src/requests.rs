extern crate reqwest;

pub fn get_user_repositories() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    //let res/*: Ip*/ = reqwest::get("https://api.github.com/users/vroussea")?.header().text()?;
    let mut res = client.get("https://api.github.com/user/repos")
        .header("Authorization", "Basic ".to_owned() + &base64::encode(b"vroussea:roussea*u59"))
        .send()?;

    let text_res = res.text()?;
    println!("Body: {:?}", text_res);

    Ok(())
}