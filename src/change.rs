use reqwest::Client;
use serde_json::Value;

use crate::loader::load_json;

pub fn post_single_user(profile_file_name: &str, bearer_token: &str) -> Result<Value, String> {
    let profile = load_json(profile_file_name)?;
    let client = Client::new()
        .post("https://change.api.dev.sso.allizom.org/v2/user")
        .json(&profile)
        .bearer_auth(bearer_token);
    let mut res: reqwest::Response = client.send().map_err(|e| format!("change.api: {}", e))?;
    res.json()
        .map_err(|e| format!("change.api → json: {} ({:?})", e, res))
}

pub fn post_lots_of_users(profile_file_name: &str, bearer_token: &str) -> Result<Value, String> {
    let profiles = load_json(profile_file_name)?;
    if let Value::Array(profiles) = profiles {
        for chunk in profiles.chunks(4) {
            let client = Client::new()
                .post("https://change.api.dev.sso.allizom.org/v2/users")
                .json(chunk)
                .bearer_auth(bearer_token);
            let mut res: reqwest::Response = client.send().map_err(|e| format!("{}", e))?;
            res.json().map_err(|e| format!("{}", e))?;
        }
    }
    Ok(json!({ "status": "all good" }))
}
