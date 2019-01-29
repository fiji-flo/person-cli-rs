use reqwest::Client;
use serde_json::Value;
use url::Url;

#[derive(Debug)]
struct Batch {
    items: Vec<Value>,
    next_page: Option<String>,
}

pub enum GetBy {
    UserId,
    Uuid,
    PrimaryEmail,
    PrimaryUsername,
}

impl GetBy {
    pub fn as_str(self: &GetBy) -> &'static str {
        match self {
            GetBy::UserId => "user_id/",
            GetBy::Uuid => "uuid/",
            GetBy::PrimaryEmail => "primary_email/",
            GetBy::PrimaryUsername => "primary_username/",
        }
    }
}

pub fn get_user(bearer_token: &str, id: &str, by: &GetBy) -> Result<Value, String> {
    let base = Url::parse("https://person.api.dev.sso.allizom.org/v2/user/")
        .map_err(|e| format!("{}", e))?;
    let url = base
        .join(by.as_str())
        .and_then(|u| u.join(id))
        .map_err(|e| format!("{}", e))?;
    let client = Client::new().get(url.as_str()).bearer_auth(bearer_token);
    let mut res: reqwest::Response = client.send().map_err(|e| format!("{}", e))?;
    if res.status().is_success() {
        res.json()
            .map_err(|e| format!("Invalid JSON from user endpoint: {}", e))
    } else {
        Err(format!("person API returned: {}", res.status()))
    }
}

pub fn get_users(bearer_token: &str) -> Result<Vec<Value>, String> {
    let mut page = None;
    let mut res = vec![];
    loop {
        let Batch { items, next_page } = get_single_user_batch(page.take(), bearer_token)?;
        res.extend(items.into_iter());
        if next_page.is_none() {
            break;
        } else {
            page = next_page
        }
    }
    Ok(res)
}

fn get_single_user_batch(
    pagination_token: Option<String>,
    bearer_token: &str,
) -> Result<Batch, String> {
    let mut client = Client::new()
        .get("https://person.api.dev.sso.allizom.org/v2/users")
        .bearer_auth(bearer_token);
    if let Some(next_page) = pagination_token {
        client = client.header("nextPage", next_page);
    }
    let mut res: reqwest::Response = client.send().map_err(|e| format!("{}", e))?;
    let mut j: serde_json::Value = res.json().map_err(|e| format!("{}", e))?;
    if let (Value::Array(items), next_page) = (j["Items"].take(), j["nextPage"].take()) {
        Ok(Batch {
            items: items,
            next_page: next_page.as_str().map(String::from),
        })
    } else {
        Err(String::from("no items / next_page"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::auth::get_access_token;

    #[test]
    fn test_get_single_batch() {
        let bearer_token = get_access_token(None).unwrap();
        let r = get_single_user_batch(None, &bearer_token);
        assert!(r.is_ok());
    }

    #[test]
    fn test_get_users() {
        let bearer_token = get_access_token(None).unwrap();
        let r = get_users(&bearer_token);
        assert!(r.is_ok());
    }
}
