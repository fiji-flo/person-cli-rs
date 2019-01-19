use std::ffi::OsString;

use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json;

use crate::auth::get_access_token;
use crate::change::post_lots_of_users;
use crate::change::post_single_user;
use crate::users::get_users;

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
                .arg(
                    Arg::with_name("user")
                        .long("user")
                        .takes_value(true)
                        .number_of_values(1)
                        .conflicts_with("users")
                        .help("Get user by id"),
                )
                .arg(
                    Arg::with_name("users")
                        .long("users")
                        .conflicts_with("user")
                        .help("Get all users"),
                ),
        )
        .subcommand(
            SubCommand::with_name("change")
                .about("Talk to change api")
                .arg(
                    Arg::with_name("user")
                        .long("user")
                        .takes_value(true)
                        .number_of_values(1)
                        .conflicts_with("users")
                        .help("Upload user profile from a json file"),
                )
                .arg(
                    Arg::with_name("users")
                        .long("users")
                        .takes_value(true)
                        .number_of_values(1)
                        .conflicts_with("user")
                        .help("upload mulitple users from a json file"),
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
    if let Some(_) = matches.value_of("user") {
        Err(String::from(r"not there yet :/"))
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
    if let Some(f) = matches.value_of("user") {
        let token = get_access_token(config)?;
        post_single_user(f, &token)
            .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)))
    } else if let Some(f) = matches.value_of("users") {
        let token = get_access_token(config)?;
        post_lots_of_users(f, &token)
            .and_then(|v| serde_json::to_string_pretty(&v).map_err(|e| format!("{}", e)))
    } else {
        Err(String::from(r"nothing to run \o/"))
    }
}
