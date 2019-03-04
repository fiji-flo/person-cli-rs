use std::ffi::OsString;

use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json;

use crate::auth::get_access_token;
use crate::change::delete_single_user;
use crate::change::post_lots_of_users;
use crate::change::post_single_user;
use crate::users::get_user;
use crate::users::get_users;
use crate::users::GetBy;

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
                .global(true)
                .takes_value(true)
                .number_of_values(1)
                .help("config file"),
        )
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
        .subcommand(SubCommand::with_name("token").about("Get the bearer token"))
        .get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<(), String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let all_matches = parse_args(itr);
    let out = if let Some(m) = all_matches.subcommand_matches("person") {
        run_person(m)
    } else if let Some(m) = all_matches.subcommand_matches("change") {
        run_change(m)
    } else if let Some(m) = all_matches.subcommand_matches("token") {
        let config = m.value_of("config");
        get_access_token(config)
    } else {
        Err(String::from("did we forget the subcommand?"))
    }?;
    println!("{}", out);

    Ok(())
}

fn run_person(matches: &ArgMatches) -> Result<String, String> {
    let config = matches.value_of("config");
    if let Some(m) = matches.subcommand_matches("user") {
        let token = get_access_token(config)?;
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
        get_user(&token, id, &get_by, m.value_of("display"))
            .and_then(|p| serde_json::to_string_pretty(&p).map_err(|e| format!("{}", e)))
    } else if matches.is_present("users") {
        let token = get_access_token(config)?;
        let profiles = get_users(&token)?;
        Ok(format!("{}", serde_json::Value::from(profiles)))
    } else {
        Err(String::from(r"nothing to run \o/"))
    }
}

fn run_change(matches: &ArgMatches) -> Result<String, String> {
    let config = matches.value_of("config");
    if let Some(json) = matches.value_of("json") {
        let token = get_access_token(config)?;
        let sign = matches.is_present("sign");
        if let Some(m) = matches.subcommand_matches("user") {
            if m.is_present("delete") {
                return delete_single_user(json, sign, &token)
                    .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)));
            }
            return post_single_user(json, sign, &token)
                .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)));
        } else if matches.subcommand_matches("users").is_some() {
            let token = get_access_token(config)?;
            return post_lots_of_users(json, sign, &token)
                .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)));
        }
    }
    Err(String::from(r"nothing to run \o/"))
}
