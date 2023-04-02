use std::collections::HashMap;

use reqwest::{header::USER_AGENT, Error};

mod types;
use types::User;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://jsonplaceholder.typicode.com/users")
        .header(USER_AGENT, "reqwest")
        .send()
        .await?
        .json::<Vec<User>>()
        .await?;

    let user_map = user_map(&res);

    println!("{:#?}", user_map);

    Ok(())
}

fn user_map(users: &Vec<User>) -> HashMap<String, &User> {
    let mut map = HashMap::new();
    for user in users {
        map.insert(user.username.clone(), user);
    }
    map
}
