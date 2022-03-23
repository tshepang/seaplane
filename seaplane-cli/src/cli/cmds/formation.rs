pub mod common;
#[cfg(feature = "unstable")]
mod configuration;
#[cfg(feature = "unstable")]
mod container_stats;
mod create;
mod delete;
mod fetch;
mod land;
mod launch;
mod list;
#[cfg(feature = "unstable")]
mod load_balance;
#[cfg(feature = "unstable")]
mod template;

pub use common::{Provider, Region};
pub use create::SeaplaneFormationCreateArgMatches;

use clap::{ArgMatches, Command};
use seaplane::api::v1::FormationsRequest;

#[cfg(feature = "unstable")]
use self::{
    configuration::SeaplaneFormationConfiguration,
    container_stats::SeaplaneFormationContainerStatistics,
    load_balance::SeaplaneFormationLoadBalance, template::SeaplaneFormationTemplate,
};
use self::{
    create::SeaplaneFormationCreate, delete::SeaplaneFormationDelete,
    fetch::SeaplaneFormationFetch, land::SeaplaneFormationLand, launch::SeaplaneFormationLaunch,
    list::SeaplaneFormationList,
};
use crate::{
    cli::{request_token, CliCommand},
    error::{CliError, Context, Result},
    Ctx,
};

/// Requests an Access Token using an API key and returns the FormationsRequest
/// The access token is only good for 60 seconds
///
/// If the name is None the only request that can be made is FormationRequest::list_names
pub fn build_request(formation_name: Option<&str>, ctx: &Ctx) -> Result<FormationsRequest> {
    let mut builder = FormationsRequest::builder();
    let formation_context = if let Some(name) = formation_name {
        builder = builder.name(name);
        format!("\n\tFormation: {name}")
    } else {
        String::new()
    };

    let token = request_token(ctx, &formation_context)?;
    builder
        .token(token)
        .build()
        .map_err(CliError::from)
        .with_context(|| {
            format!("Context: failed to build /formations endpoint request{formation_context}\n")
        })
}

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormation;

impl SeaplaneFormation {
    pub fn command() -> Command<'static> {
        #[cfg_attr(not(feature = "unstable"), allow(unused_mut))]
        let mut app = Command::new("formation")
            .about("Operate on Seaplane Formations")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(SeaplaneFormationCreate::command())
            .subcommand(SeaplaneFormationDelete::command())
            .subcommand(SeaplaneFormationFetch::command())
            .subcommand(SeaplaneFormationLand::command())
            .subcommand(SeaplaneFormationLaunch::command())
            .subcommand(SeaplaneFormationList::command());

        #[cfg(feature = "unstable")]
        {
            app = app
                .subcommand(SeaplaneFormationConfiguration::command())
                .subcommand(SeaplaneFormationContainerStatistics::command())
                .subcommand(SeaplaneFormationLoadBalance::command())
                .subcommand(SeaplaneFormationTemplate::command())
        }

        app
    }
}

impl CliCommand for SeaplaneFormation {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("create", m)) => Some((Box::new(SeaplaneFormationCreate), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneFormationDelete), m)),
            Some(("fetch-remote", m)) => Some((Box::new(SeaplaneFormationFetch), m)),
            Some(("land", m)) => Some((Box::new(SeaplaneFormationLand), m)),
            Some(("launch", m)) => Some((Box::new(SeaplaneFormationLaunch), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneFormationList), m)),
            #[cfg(feature = "unstable")]
            Some(("configuration", m)) => Some((Box::new(SeaplaneFormationConfiguration), m)),
            #[cfg(feature = "unstable")]
            Some(("container-statistics", m)) => {
                Some((Box::new(SeaplaneFormationContainerStatistics), m))
            }
            #[cfg(feature = "unstable")]
            Some(("load-balance", m)) => Some((Box::new(SeaplaneFormationLoadBalance), m)),
            #[cfg(feature = "unstable")]
            Some(("template", m)) => Some((Box::new(SeaplaneFormationTemplate), m)),
            _ => None,
        }
    }

    fn update_ctx(&self, _matches: &ArgMatches, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
}
