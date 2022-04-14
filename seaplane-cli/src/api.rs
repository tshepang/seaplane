//! Wrapping seaplane SDK calls with things like CLI specific contexts, errors, etc.

use seaplane::api::{v1::formations::FormationsRequest, AccessToken, TokenRequest};

use crate::{
    error::{CliError, Context, Result},
    ops::formation::{Formation, FormationConfiguration, Formations},
    printer::Color,
};

/// Makes a request against the `/token` endpoint of FlightDeck using the discovered API key and
/// returns the short lived Access token (JWT). The access token is only good for 60 seconds
pub fn request_token(api_key: &str, context: &str) -> Result<String> {
    Ok(request_token_json(api_key, context)?.token)
}

/// Follows the same process as `request_token` but instead of just returning a raw JWT string,
/// this function returns the full response deserialized from JSON which contains other fields such
/// as the tenant name, subdomain, etc.
pub fn request_token_json(api_key: &str, context: &str) -> Result<AccessToken> {
    TokenRequest::builder()
        .api_key(api_key)
        .build()
        .map_err(CliError::from)
        .with_context(|| format!("Context: failed to build Access Token request{context}\n"))?
        .access_token_json()
        .map_err(CliError::from)
        .with_context(|| format!("Context: failed to retrieve an Access Token{context}\n"))
}

/// Builds a FormationsRequest and requests an access token using the given API key. If the
/// `formation_name` is supplied context will be added to any potential error message saying which
/// Formation the request failed from.
///
/// If the `formation_name` is `None` it should be noted that the only request that can be made
/// without error is `FormationsRequest::list_names`
pub fn build_formations_request(
    formation_name: Option<&str>,
    api_key: &str,
) -> Result<FormationsRequest> {
    let mut builder = FormationsRequest::builder();
    let formation_context = if let Some(name) = formation_name {
        builder = builder.name(name);
        format!("\n\tFormation: {name}")
    } else {
        String::new()
    };

    let token = request_token(api_key, &formation_context)?;
    builder
        .token(token)
        .build()
        .map_err(CliError::from)
        .with_context(|| {
            format!("Context: failed to build /formations endpoint request{formation_context}\n")
        })
}

/// Retrieves all Formations and their Formation Configurations (both active and inactive) from the
/// Compute API. Makes multiple calls against the API to gather the info and returns a `Formations`
/// struct.
///
/// It should be noted that the local IDs associated with all the items in the Formations struct
/// are generated unique after retrieval from the compute API. i.e. they do not match anything
/// existing in the local DB even if the contents are otherwise identical.
pub fn get_all_formations<S: AsRef<str>>(
    api_key: &str,
    formation_names: &[S],
) -> Result<Formations> {
    let mut formations = Formations::default();
    // TODO: We're requesting tons of new tokens...maybe we could do multiple per and just
    // retry on error?
    for name in formation_names {
        let name = name.as_ref();
        let mut formation = Formation::new(name);
        let list_cfg_uuids_req = build_formations_request(Some(name), api_key)?;

        let cfg_uuids = list_cfg_uuids_req
            .list_configuration_ids()
            .map_err(CliError::from)
            .context("Context: failed to retrieve Formation Configuration IDs\n")?;
        let active_cfgs_req = build_formations_request(Some(name), api_key)?;
        let active_cfgs = active_cfgs_req
            .get_active_configurations()
            .map_err(CliError::from)
            .context("Context: failed to retrieve Active Formation Configurations\n")?;

        for uuid in cfg_uuids.into_iter() {
            let get_cfgs_req = build_formations_request(Some(name), api_key)?;
            let cfg_model = get_cfgs_req
                .get_configuration(uuid)
                .map_err(CliError::from)
                .context("Context: failed to retrieve Formation Configuration\n\tUUID: ")
                .with_color_context(|| (Color::Yellow, format!("{uuid}\n")))?;

            let cfg = FormationConfiguration::with_uuid(uuid, cfg_model);
            let is_active = active_cfgs.iter().any(|ac| ac.uuid() == &uuid);
            formation.local.insert(cfg.id);
            if is_active {
                formation.in_air.insert(cfg.id);
            } else {
                formation.grounded.insert(cfg.id);
            }
            formations.configurations.push(cfg);
        }

        formations.formations.push(formation);
    }

    Ok(formations)
}

/// Return either a list of all known formation names, or a list of the name passed in. This is
/// used when the CLI supports either doing something to all formations, or just a single one that
/// is passed in by the user.
pub fn get_formation_names(api_key: &str, name: Option<&str>) -> Result<Vec<String>> {
    Ok(if let Some(name) = name {
        vec![name.to_owned()]
    } else {
        // First download all formation names
        let formations_request = build_formations_request(None, api_key)?;
        formations_request
            .list_names()
            .map_err(CliError::from)
            .context("Context: failed to retrieve Formation Instance names\n")?
            .into_inner()
    })
}
