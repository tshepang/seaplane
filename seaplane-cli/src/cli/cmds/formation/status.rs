use clap::{value_parser, ArgMatches, Command};

use crate::{
    api::FormationsReq,
    cli::{
        cmds::formation::SeaplaneFormationFetch,
        validator::{validate_formation_name, validate_name_id},
        CliCommand,
    },
    error::Result,
    ops::formation::FormationStatus,
    printer::{Output, Pb},
    Ctx, OutputFormat,
};

static LONG_ABOUT: &str = "Show the status of a remote Formation Instance

This command will display the status of one or more Formation Instances such as how many actual
containers are running compared to the minimum and maximums per Flight Plan that the configuration
defines.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationStatus;

impl SeaplaneFormationStatus {
    pub fn command() -> Command<'static> {
        let validator = |s: &str| validate_name_id(validate_formation_name, s);
        Command::new("status")
            .long_about(LONG_ABOUT)
            .about("Show the status of a remote Formation Instance")
            .arg(
                arg!(formation = ["NAME|ID"])
                    .validator(validator)
                    .help("The name or ID of the Formation to check, must be unambiguous"),
            )
            .arg(
                arg!(--format =["FORMAT"=>"table"])
                    .value_parser(value_parser!(OutputFormat))
                    .help("Change the output format"),
            )
            .arg(arg!(--("no-fetch")).help("Skip fetching and synchronizing of remote instances"))
    }
}

impl CliCommand for SeaplaneFormationStatus {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let old_stateless = ctx.args.stateless;

        if ctx.args.fetch {
            // Make sure the local DB is up to date, but don't persist the data. Also keep track of
            // if we were originally in stateless mode or not so we can go back after
            // this call.
            ctx.internal_run = true;
            ctx.disable_pb = ctx.args.out_format == OutputFormat::Json;
            ctx.args.stateless = true;
            let old_name = ctx.args.name_id.take();
            SeaplaneFormationFetch.run(ctx)?;
            ctx.args.name_id = old_name;
            ctx.internal_run = false;
            ctx.args.stateless = old_stateless;
        }

        let pb = Pb::new(ctx);

        let names = if let Some(name) = ctx.args.name_id.as_deref() {
            vec![name]
        } else {
            ctx.db.formations.remote_names()
        };

        let mut statuses: Vec<FormationStatus> = Vec::new();

        let mut req = FormationsReq::new_delay_token(ctx)?;
        for name in names {
            pb.set_message(format!("Gathering {name} container info..."));
            req.set_name(name)?;
            let mut f_status = FormationStatus::new(name);
            for container in req.get_containers()?.iter() {
                if let Some(cfg) = ctx
                    .db
                    .formations
                    .get_configuration_by_uuid(container.configuration_id)
                {
                    if let Some(flight) = cfg.get_flight(&container.flight_name) {
                        f_status.add_container(container, flight.minimum(), flight.maximum());
                    }
                }
            }
            // TODO it stinks that we have to do this here and it's not automatic
            f_status.update_status();
            statuses.push(f_status);
        }

        pb.finish_and_clear();

        match ctx.args.out_format {
            OutputFormat::Json => statuses.print_json(ctx)?,
            OutputFormat::Table => statuses.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        ctx.args.name_id = matches
            .get_one::<String>("formation")
            .map(ToOwned::to_owned);
        ctx.args.fetch = !matches.contains_id("no-fetch");
        Ok(())
    }
}
