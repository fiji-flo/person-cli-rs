use chrono::DateTime;
use chrono::Utc;
use cis_profile::schema::Display;
use cis_profile::schema::Metadata;
use cis_profile::schema::Profile;
use failure::Error;

const NULL_PROFILE: &str = include_str!("data/user_profile_null.json");

fn update_metadata(metadata: &mut Metadata, display: Option<Display>, now: &DateTime<Utc>) {
    metadata.last_modified = *now;
    metadata.created = *now;
    metadata.display = display;
}

pub fn empty_profile(typ: &str) -> Result<String, String> {
    let p = match typ {
        "null" => {
            serde_json::from_str(NULL_PROFILE).map_err(|e| format!("error reading skeleton {}", e))
        }
        "rust" => Ok(Profile::default()),
        _ => Err(String::from("only: null, create, rust supported")),
    };
    p.and_then(|p| {
        serde_json::to_string_pretty(&p).map_err(|e| format!("unable to print profile: {}", e))
    })
}

pub fn create_new_user(
    user_id: String,
    primary_email: String,
    first_name: String,
    last_name: Option<String>,
) -> Result<Profile, Error> {
    let now = &Utc::now();
    let mut p: Profile = serde_json::from_str(NULL_PROFILE)?;
    p.primary_email.value = Some(primary_email);
    update_metadata(&mut p.primary_email.metadata, Some(Display::Private), now);
    p.user_id.value = Some(user_id);
    update_metadata(&mut p.user_id.metadata, None, now);
    p.active.value = Some(true);
    update_metadata(&mut p.active.metadata, None, now);
    p.first_name.value = Some(first_name);
    update_metadata(&mut p.first_name.metadata, Some(Display::Private), now);
    if last_name.is_some() {
        p.last_name.value = last_name;
        update_metadata(&mut p.last_name.metadata, Some(Display::Private), now);
    }
    Ok(p)
}
