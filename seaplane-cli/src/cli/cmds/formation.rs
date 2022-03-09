mod common;
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

use clap::{Parser, Subcommand};
use seaplane::api::{v1::FormationsRequest, TokenRequest};

use self::{
    common::SeaplaneFormationCommonArgs, create::SeaplaneFormationCreateArgs,
    delete::SeaplaneFormationDeleteArgs, fetch::SeaplaneFormationFetchArgs,
    land::SeaplaneFormationLandArgs, launch::SeaplaneFormationLaunchArgs,
    list::SeaplaneFormationListArgs,
};
#[cfg(feature = "unstable")]
use self::{
    configuration::SeaplaneFormationConfigurationArgs,
    container_stats::SeaplaneFormationContainerStatisticsArgs,
    load_balance::SeaplaneFormationLoadBalanceArgs, template::SeaplaneFormationTemplateArgs,
};
use crate::{
    error::{CliError, CliErrorKind, Context, Result},
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
        format!("\n\tFormation: {}", name)
    } else {
        String::new()
    };

    let token = TokenRequest::builder()
        .api_key(
            // TODO: add context
            ctx.api_key
                .as_ref()
                .ok_or_else(|| CliErrorKind::MissingApiKey.into_err())?,
        )
        .build()
        .map_err(CliError::from)
        .with_context(|| {
            format!(
                "Context: failed to build Access Token request{}\n",
                formation_context
            )
        })?
        .access_token()
        .map_err(CliError::from)
        .with_context(|| {
            format!(
                "Context: failed to retrieve an Access Token{}\n",
                formation_context
            )
        })?;
    builder
        .token(token)
        .build()
        .map_err(CliError::from)
        .with_context(|| {
            format!(
                "Context: failed to build /formations endpoint request{}\n",
                formation_context
            )
        })
}

/// Operate on Seaplane Formations
#[derive(Parser)]
pub struct SeaplaneFormationArgs {
    #[clap(subcommand)]
    cmd: SeaplaneFormationCmds,
}

impl SeaplaneFormationArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        use SeaplaneFormationCmds::*;
        self.update_ctx(ctx)?;

        match &self.cmd {
            #[cfg(feature = "unstable")]
            Configuration(args) => args.run(ctx),
            #[cfg(feature = "unstable")]
            ContainerStatistics(args) => args.run(ctx),
            Create(args) => args.run(ctx),
            Delete(args) => args.run(ctx),
            FetchRemote(args) => args.run(ctx),
            Land(args) => args.run(ctx),
            Launch(args) => args.run(ctx),
            List(args) => args.run(ctx),
            #[cfg(feature = "unstable")]
            LoadBalance(args) => args.run(ctx),
            #[cfg(feature = "unstable")]
            Template(args) => args.run(ctx),
        }
    }

    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum SeaplaneFormationCmds {
    #[cfg(feature = "unstable")]
    Configuration(SeaplaneFormationConfigurationArgs),
    #[cfg(feature = "unstable")]
    ContainerStatistics(SeaplaneFormationContainerStatisticsArgs),
    Create(Box<SeaplaneFormationCreateArgs>),
    Delete(SeaplaneFormationDeleteArgs),
    FetchRemote(SeaplaneFormationFetchArgs),
    Land(SeaplaneFormationLandArgs),
    Launch(SeaplaneFormationLaunchArgs),
    List(SeaplaneFormationListArgs),
    #[cfg(feature = "unstable")]
    Template(SeaplaneFormationTemplateArgs),
    #[cfg(feature = "unstable")]
    LoadBalance(SeaplaneFormationLoadBalanceArgs),
}
