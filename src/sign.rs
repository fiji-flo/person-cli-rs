use cis_profile::crypto::SecretStore;
use cis_profile::schema::Profile;
use cis_profile::utils::sign_full_profile;
use serde_json::Value;
use std::env;

fn get_store() -> Result<SecretStore, String> {
    if let (
        Ok(mozillians_key_ssm_name),
        Ok(hris_key_ssm_name),
        Ok(ldap_key_ssm_name),
        Ok(cis_key_ssm_name),
        Ok(access_provider_key_ssm_name),
    ) = (
        env::var("CIS_SSM_MOZILLIANSORG_KEY"),
        env::var("CIS_SSM_HRIS_KEY"),
        env::var("CIS_SSM_LDAP_KEY"),
        env::var("CIS_SSM_CIS_KEY"),
        env::var("CIS_SSM_ACCESS_PROVIDER_KEY"),
    ) {
        SecretStore::from_ssm_iter(vec![
            (String::from("mozilliansorg"), mozillians_key_ssm_name),
            (String::from("hris"), hris_key_ssm_name),
            (String::from("ldap"), ldap_key_ssm_name),
            (String::from("cis"), cis_key_ssm_name),
            (
                String::from("access_provider"),
                access_provider_key_ssm_name,
            ),
        ])
    } else {
        Err(String::from("missing CIS_SSM_XXX environment variables"))
    }
}

pub fn sign_profile(profile_v: Value) -> Result<Value, String> {
    let mut profile: Profile =
        serde_json::from_value(profile_v).map_err(|e| format!("unable to read profile: {}", e))?;
    let store = get_store()?;
    sign_full_profile(&mut profile, &store)?;
    serde_json::to_value(profile).map_err(|e| format!("unable to convert profile: {}", e))
}
