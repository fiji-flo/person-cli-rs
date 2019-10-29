use chrono::SecondsFormat;
use chrono::Utc;
use cis_profile::schema::Display;
use cis_profile::schema::Metadata;
use cis_profile::schema::Profile;
use failure::Error;

const CREATE_PROFILE: &str = include_str!("data/user_profile_null_create.json");

fn update_metadata(metadata: &mut Metadata, display: Option<Display>, now: &str) {
    metadata.last_modified = String::from(now);
    metadata.created = String::from(now);
    metadata.display = display;
}

pub fn create_new_user(
    user_id: String,
    primary_email: String,
    first_name: String,
    last_name: Option<String>,
) -> Result<Profile, Error> {
    let now = &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let mut p: Profile = serde_json::from_str(CREATE_PROFILE)?;
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
