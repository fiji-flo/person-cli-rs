use reqwest::Client;

use crate::loader::load_json;

struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
}

pub fn get_access_token(config_file: Option<&str>) -> Result<String, String> {
    let client_config = read_client_config(config_file.unwrap_or_else(|| ".person-cli.json"))?;
    let payload = json!(
        {
            "client_id": client_config.client_id,
            "client_secret": client_config.client_secret,
            "audience": client_config.audience,
            "grant_type": "client_credentials",
            "scopes": "read:fullprofile display:all"
        }
    );
    let client = Client::new();
    let mut res: reqwest::Response = client
        .post("https://auth.mozilla.auth0.com/oauth/token")
        .json(&payload)
        .send()
        .map_err(|e| format!("can't get token: {}", e))?;
    let j: serde_json::Value = res
        .json()
        .map_err(|e| format!("can't parse token: {}", e))?;
    j["access_token"]
        .as_str()
        .map(|s| s.to_owned())
        .ok_or_else(|| String::from("no token :/"))
}

fn read_client_config(config_file: &str) -> Result<ClientConfig, String> {
    let config = load_json(config_file)?;
    let client_id = if let Some(client_id) = config["client_id"].as_str() {
        String::from(client_id)
    } else {
        return Err(String::from("missing client_id in config"));
    };
    let client_secret = if let Some(client_secret) = config["client_secret"].as_str() {
        String::from(client_secret)
    } else {
        return Err(String::from("missing client_secret in config"));
    };
    let audience = if let Some(audience) = config["audience"].as_str() {
        String::from(audience)
    } else {
        return Err(String::from("missing audience in config"));
    };
    Ok(ClientConfig {
        client_id,
        client_secret,
        audience,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_access_token() {
        let r = get_access_token(None);
        assert!(r.is_ok());
    }
}
