use clap::Parser;

use crate::{
    cli::{cmds::formation::build_request, errors, validator::validate_name_id},
    error::{Context, Result},
    fs::{FromDisk, ToDisk},
    ops::formation::Formations,
    Ctx,
};

/// Land (Stop) all configurations of a Formation
#[derive(Parser)]
#[clap(visible_alias = "stop")]
pub struct SeaplaneFormationLandArgs {
    #[clap(value_name = "NAME|ID", validator = validate_name_id)]
    formation: String,

    /// Stop all matching Formations even when FORMATION is ambiguous
    #[clap(short, long, conflicts_with = "exact")]
    all: bool,

    /// the given FORMATION must be an exact match
    #[clap(short = 'x', long, conflicts_with = "all")]
    exact: bool,
}

impl SeaplaneFormationLandArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        // Load the known Formations from the local JSON "DB"
        let formations_file = ctx.formations_file();
        let mut formations: Formations = FromDisk::load(&formations_file)?;

        // Get the indices of any formations that match the given name/ID
        let indices = if self.exact {
            formations.formation_indices_of_matches(&self.formation)
        } else {
            formations.formation_indices_of_left_matches(&self.formation)
        };

        match indices.len() {
            0 => errors::no_matching_item(self.formation.clone(), self.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !self.all {
                    errors::ambiguous_item(self.formation.clone(), true)?;
                }
            }
        }

        for idx in indices {
            // re unwrap: the indices returned came from Formations so they have to be valid
            let formation = formations.get_formation_mut(idx).unwrap();

            // re unwrap: We got the formation from the local DB so it has to have a name
            let stop_req = build_request(Some(formation.name.as_ref().unwrap()), ctx)?;
            stop_req.stop()?;

            // Move all configurations from in air to grounded
            let ids: Vec<_> = formation.in_air.drain().collect();
            for id in ids {
                formation.grounded.insert(id);
            }

            // Write out an entirely new JSON file with the new Formation included
            formations
                .persist()
                .with_context(|| format!("Path: {:?}\n", ctx.formations_file()))?;

            cli_print!("Successfully Landed Formation '");
            cli_print!(@Green, "{}", &self.formation);
            cli_println!("'");
        }

        Ok(())
    }
}
