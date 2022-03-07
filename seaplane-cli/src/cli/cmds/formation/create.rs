use clap::Parser;

use crate::{
    cli::cmds::formation::{build_request, SeaplaneFormationCommonArgs},
    error::{CliErrorKind, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::formation::{Formation, FormationConfiguration, Formations},
    printer::{Color, Printer},
    Ctx,
};

/// Create a Seaplane Formation
#[derive(Parser)]
#[clap(visible_aliases = &["add"], override_usage =
    "seaplane formation create [OPTIONS]
    seaplane formation create --flight=SPEC... [FORMATION CFG OPTIONS]")]
pub struct SeaplaneFormationCreateArgs {
    // So we don't have to define the same args over and over with commands that use the same ones
    #[clap(flatten)]
    shared: SeaplaneFormationCommonArgs,

    /// Send this formation to Seaplane immediately (requires a Formation configuration) (implies
    /// --take-off, if that is not the desired state use --no-take-off)
    #[clap(long, overrides_with = "no-deploy")]
    deploy: bool,

    /// Do *not* send this formation to Seaplane immediately
    #[clap(long, overrides_with = "no-deploy")]
    no_deploy: bool,

    /// Override any existing Formation with the same NAME
    #[clap(long)]
    force: bool,
}

impl SeaplaneFormationCreateArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;

        let formation_ctx = ctx.formation_ctx();

        // Load the known formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let mut formations: Formations = FromDisk::load(&formations_file)?;

        // Check for duplicates and suggest `seaplane formation edit`
        let name = &formation_ctx.name;
        if formations.contains_name(name) {
            // TODO: We should check if these ones we remove are referenced remote or not

            if !ctx.force {
                return Err(CliErrorKind::DuplicateName(name.to_owned())
                    .into_err()
                    .context("(hint: try '")
                    .color_context(Color::Green, format!("seaplane formation edit {}", &name))
                    .context("' instead)\n"));
            }

            // We have duplicates, but the user passed --force. So first we remove the existing
            // formations and "re-add" them

            // TODO: if more than one formation has the exact same name, we remove them all; that's
            // *probably* what we want? But more thought should go into this...
            formations.remove_name(name);
        }

        // Add the new formation
        let mut new_formation = Formation::new(&formation_ctx.name);
        let mut cfg_id = None;

        if let Some(cfg) = formation_ctx.configuration_model(ctx)? {
            let formation_cfg = FormationConfiguration::new(cfg);
            // TODO: if active / deployed add to appropriate in_air / grounded
            new_formation.local.insert(formation_cfg.id);
            cfg_id = Some(formation_cfg.id);
            formations.configurations.push(formation_cfg)
        }

        let id = new_formation.id.to_string();
        formations.formations.push(new_formation);

        // Write out an entirely new JSON file with the new Formation included
        formations
            .persist()
            .with_context(|| format!("Path: {:?}\n", ctx.formations_file()))?;

        cli_print!("Successfully created Formation '");
        cli_print!(@Green, "{}", &formation_ctx.name);
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id[..8]);
        cli_println!("'");

        if let Some(cfg_id) = cfg_id {
            cli_print!("Successfully created Formation Configuration with ID '");
            cli_print!(@Green, "{}", &cfg_id.to_string()[..8]);
            cli_println!("'");

            if formation_ctx.deploy {
                let create_req = build_request(Some(&formation_ctx.name), ctx)?;
                let cfg_uuids = create_req.create(
                    formation_ctx.configuration_model(ctx)?.unwrap(),
                    formation_ctx.take_off,
                )?;
                for uuid in cfg_uuids.into_iter() {
                    formations.add_uuid(&cfg_id, uuid);
                }

                if formation_ctx.take_off {
                    formations.add_in_air_by_name(&formation_ctx.name, cfg_id);
                } else {
                    formations.add_grounded_by_name(&formation_ctx.name, cfg_id);
                }
            }
        }

        Ok(())
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.force = self.force;
        ctx.formation.init(self.shared.formation_ctx(ctx)?);
        let mut fctx = ctx.formation_ctx();
        fctx.deploy = self.deploy;
        if self.deploy && !self.shared.no_take_off {
            fctx.take_off = true;
        }

        Ok(())
    }
}
