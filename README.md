# A basic cli-client for [CIS](https://github.com/mozilla-iam/cis/)

## Installation

```
cargo install --git https://github.com/fiji-flo/person-cli-rs.git
```

## Configuration

`-c CONFIGFILE` with a config JSON like:
```json
{
  "cis": {
    "client_config": {
      "client_id": "…",
      "client_secret": "…",
      "audience": "api.dev.sso.allizom.org",
      "token_endpoint": "https://auth.mozilla.auth0.com/oauth/token",
      "scopes": "read:fullprofile display:all"
    },
    "sign_keys": {
      "source": "file",
      "mozilliansorg_key": "/keys/mozilliansorg",
      "access_provider_key": "/keys/access_provider",
      "hris_key": "/keys/hris", 
      "cis_key": "/keys/cis", 
      "ldap_key": "/keys/ldap" 
    },
    "verify_keys": {
      "source": "none"
    },
    "change_api_user_endpoint": "https://change.api.dev.sso.allizom.org/v2/user",
    "change_api_users_endpoint": "https://change.api.dev.sso.allizom.org/v2/users",
    "person_api_user_endpoint": "https://person.api.dev.sso.allizom.org/v2/user/",
    "person_api_users_endpoint": "https://person.api.dev.sso.allizom.org/v2/users"
  }
}
```


## Usage

```
$ person_cli --help
person-cli 0.1.0
Florian Merz <fmerz@mozilla.com>
Get them all

USAGE:
    person_cli [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    set the config

SUBCOMMANDS:
    change     Talk to change api
    create     Create a new user
    help       Prints this message or the help of the given subcommand(s)
    person     Talk to person api
    profile    Print an empty profile
    sign       Sign an print a profile
    token      Print the access token

```

### Getting a profile and piping through jq

```
person_cli -c settings.json person user --username "fiji" | jq . -C | less -R
```