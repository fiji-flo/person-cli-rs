use crate::name::ExternalFileName;
use crate::name::InternalFileName;
use crate::resize::Avatars;
use chrono::SecondsFormat;
use chrono::Utc;
use cis_profile::schema::Profile;
use cis_profile::schema::PublisherAuthority;
use failure::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

pub fn has_picture_path(profile: Profile) -> Option<String> {
    if let (Some(pic), Some(user_id)) = (profile.picture.value, profile.user_id.value) {
        if pic.starts_with("https://s3.amazonaws.com/") {
            return Some(format!("{}.jpg", user_id));
        }
    }
    None
}

pub fn process_picture(profile: Profile, in_path: &str, out_path: &str) -> Option<Profile> {
    println!(
        "processing: {}",
        profile.primary_email.value.unwrap_or_default()
    );
    if let (Some(pic), Some(uuid), Some(user_id)) = (
        profile.picture.value,
        profile.uuid.value,
        profile.user_id.value.clone(),
    ) {
        if pic.starts_with("https://s3.amazonaws.com/") {
            match Avatars::new(&PathBuf::from(in_path).join(&format!("{}.jpg", user_id))) {
                Ok(avatar) => {
                    let name = InternalFileName::from_uuid_and_display(&uuid, "staff").to_string();
                    match write_files(&avatar, out_path, &name) {
                        Ok(_) => {
                            let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
                            let mut update = Profile::default();
                            update.active = profile.active;
                            update.user_id = profile.user_id;
                            update.picture.signature.publisher.name =
                                PublisherAuthority::Mozilliansorg;
                            update.picture.metadata.last_modified = now;
                            update.picture.metadata.verified = true;
                            update.picture.metadata.created = profile.picture.metadata.created;
                            update.picture.value = Some(format!(
                                "/avatar/get/id/{}",
                                ExternalFileName::from_uuid_and_display(&uuid, "staff").filename()
                            ));
                            return Some(update);
                        }
                        Err(e) => eprintln!("unable to save file: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("unable to read file: {}", e);
                }
            }
        }
    }
    None
}

fn write_files(avatar: &Avatars, out_path: &str, name: &str) -> Result<(), Error> {
    write_to_file(&avatar.raw, PathBuf::from(out_path).join("raw").join(name))?;
    write_to_file(&avatar.x40, PathBuf::from(out_path).join("40").join(name))?;
    write_to_file(&avatar.x100, PathBuf::from(out_path).join("100").join(name))?;
    write_to_file(&avatar.x264, PathBuf::from(out_path).join("264").join(name))
}

fn write_to_file(buf: &[u8], path: PathBuf) -> Result<(), Error> {
    let f = File::create(path)?;
    {
        let mut writer = BufWriter::new(f);
        writer.write(buf)?;
    }
    Ok(())
}
