use std::ffi::OsString;

use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json;

use crate::loader::load_json;
use crate::settings;
use cis_client::client::CisClient;
use cis_client::client::CisClientTrait;
use cis_client::client::GetBy;
use cis_profile::schema::Profile;
use cis_profile::utils::sign_full_profile;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("person-cli")
        .about("Get them all")
        .version(VERSION)
        .author("Florian Merz <fmerz@mozilla.com>")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .number_of_values(1)
                .help("set the config"),
        )
        .subcommand(SubCommand::with_name("token").about("Print the access token"))
        .subcommand(
            SubCommand::with_name("person")
                .about("Talk to person api")
                .subcommand(
                    SubCommand::with_name("user")
                        .about("Query for a specific user")
                        .arg(
                            Arg::with_name("user_id")
                                .long("user_id")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("Get user by user id")
                                .conflicts_with_all(&["uuid", "email", "username"]),
                        )
                        .arg(
                            Arg::with_name("uuid")
                                .long("uuid")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("Get user by uuid")
                                .conflicts_with_all(&["user_id", "email", "username"]),
                        )
                        .arg(
                            Arg::with_name("email")
                                .long("email")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("Get user by primary email")
                                .conflicts_with_all(&["user_id", "uuid", "username"]),
                        )
                        .arg(
                            Arg::with_name("username")
                                .long("username")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("Get user by primary username")
                                .conflicts_with_all(&["user_id", "uuid", "email"]),
                        )
                        .arg(
                            Arg::with_name("display")
                                .long("display")
                                .short("d")
                                .takes_value(true)
                                .number_of_values(1)
                                .help("filter by DISPLAY level"),
                        ),
                )
                .subcommand(SubCommand::with_name("users").about("Query for a specific user")),
        )
        .subcommand(
            SubCommand::with_name("sign")
                .about("Sign an print a profile")
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .short("j")
                        .required(true)
                        .takes_value(true)
                        .number_of_values(1)
                        .help("the json file"),
                )
                .arg(
                    Arg::with_name("pretty")
                        .long("pretty")
                        .short("p")
                        .help("pretty print the profile"),
                )
        )
        .subcommand(
            SubCommand::with_name("change")
                .about("Talk to change api")
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .short("j")
                        .required(true)
                        .takes_value(true)
                        .number_of_values(1)
                        .help("the json file"),
                )
                .arg(
                    Arg::with_name("sign")
                        .long("sign")
                        .short("s")
                        .help("sign the profile"),
                )
                .subcommand(
                    SubCommand::with_name("user")
                        .about("Upload user profile from a json file")
                        .arg(
                            Arg::with_name("delete")
                                .long("delete")
                                .short("d")
                                .help("delete the profile"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("users")
                        .about("Upload lots of user profiles from a json file"),
                ),
        )
        .get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<(), String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let all_matches = parse_args(itr);
    let s = settings::Settings::new(all_matches.value_of("config"))
        .map_err(|e| format!("unable to load settings: {}", e))?;
    let cis_client = CisClient::from_settings(&s.cis).map_err(|e| e.to_string())?;
    let out = if let Some(m) = all_matches.subcommand_matches("person") {
        run_person(m, cis_client)
    } else if let Some(m) = all_matches.subcommand_matches("change") {
        run_change(m, cis_client)
    } else if let Some(m) = all_matches.subcommand_matches("sign") {
        run_sign(m, cis_client)
    } else if let Some(_) = all_matches.subcommand_matches("token") {
        cis_client.bearer_token().map_err(|e| e.to_string())
    } else {
        Err(String::from("did we forget the subcommand?"))
    }?;
    println!("{}", out);

    Ok(())
}

fn run_sign(matches: &ArgMatches, cis_client: CisClient) -> Result<String, String> {
    if let Some(json) = matches.value_of("json") {
        let mut profile: Profile = serde_json::from_value(load_json(json)?)
            .map_err(|e| format!("unable to deserialize profile: {}", e))?;
                sign_full_profile(&mut profile, cis_client.get_secret_store())
                    .map_err(|e| e.to_string())?;
        if matches.is_present("pretty") {
            return serde_json::to_string_pretty(&profile).map_err(|e| format!("{}", e))
        }
        return serde_json::to_string(&profile).map_err(|e| format!("{}", e))
    }
    Err(String::from("no profile provied"))
}

fn run_person(matches: &ArgMatches, cis_client: CisClient) -> Result<String, String> {
    if let Some(m) = matches.subcommand_matches("user") {
        let (id, get_by) = if let Some(id) = m.value_of("user_id") {
            (id, GetBy::UserId)
        } else if let Some(id) = m.value_of("uuid") {
            (id, GetBy::Uuid)
        } else if let Some(id) = m.value_of("email") {
            (id, GetBy::PrimaryEmail)
        } else if let Some(id) = m.value_of("username") {
            (id, GetBy::PrimaryUsername)
        } else {
            return Err(String::from("user command needs a least one argument"));
        };
        cis_client
            .get_user_by(id, &get_by, m.value_of("display"))
            .map_err(|e| e.to_string())
            .and_then(|p| serde_json::to_string_pretty(&p).map_err(|e| format!("{}", e)))
    } else if matches.is_present("users") {
        let profiles = cis_client
            .get_users_iter(None)
            .map_err(|e| e.to_string())?
            .flatten()
            .flatten()
            .collect::<Vec<Profile>>();
        Ok(serde_json::to_string_pretty(&profiles)
            .map_err(|e| format!("unable to serialize profiles: {}", e))?)
    } else {
        Err(String::from(r"nothing to run \o/"))
    }
}

fn run_change(matches: &ArgMatches, cis_client: CisClient) -> Result<String, String> {
    if let Some(json) = matches.value_of("json") {
        if let Some(m) = matches.subcommand_matches("user") {
            let mut profile: Profile = serde_json::from_value(load_json(json)?)
                .map_err(|e| format!("unable to deserialize profile: {}", e))?;
            let id = profile
                .user_id
                .value
                .clone()
                .ok_or_else(|| String::from("no user_id set"))?;
            let sign = matches.is_present("sign");
            if sign {
                sign_full_profile(&mut profile, cis_client.get_secret_store())
                    .map_err(|e| e.to_string())?;
                if m.is_present("delete") {
                    return cis_client
                        .delete_user(&id, profile)
                        .map_err(|e| e.to_string())
                        .and_then(|v| {
                            serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e))
                        });
                }
            }
            return cis_client
                .update_user(&id, profile)
                .map_err(|e| e.to_string())
                .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)));
        } else if matches.subcommand_matches("users").is_some() {
            let mut profiles: Vec<Profile> = serde_json::from_value(load_json(json)?)
                .map_err(|e| format!("unable to deserialize profile: {}", e))?;
            let sign = matches.is_present("sign");
            if sign {
                for p in &mut profiles {
                    sign_full_profile(p, cis_client.get_secret_store())
                        .map_err(|e| e.to_string())?;
                }
            }
            return cis_client
                .update_users(&profiles)
                .map_err(|e| e.to_string())
                .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)));
        }
    }
    Err(String::from(r"nothing to run \o/"))
}
