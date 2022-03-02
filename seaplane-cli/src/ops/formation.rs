use std::{collections::HashMap, io::Write};

use seaplane::api::v1::formations::FormationConfiguration as FormationConfigurationModel;
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;

use crate::{
    context::Ctx,
    error::{CliError, Result},
    printer::Output,
};

// Newtype wrapper around the `FormationConfiguration` model.
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct FormationConfiguration(FormationConfigurationModel);

impl FormationConfiguration {
    fn is_active(&self) -> bool {
        todo!("impl FormationConfiguration::is_active")
    }
}

#[derive(Deserialize, Serialize)]
pub struct Formations {
    // (LocalID, Formation)
    #[serde(flatten)]
    inner: HashMap<String, Formation>,
}

impl Output for Formations {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);

        Ok(())
    }

    fn print_table(&self, _ctx: &Ctx) -> Result<()> {
        let buf = Vec::new();
        let mut tw = TabWriter::new(buf);
        writeln!(
            tw,
            "LOCAL ID\tNAME\tCONFIGURATIONS\tLOCAL STATUS\tREMOTE STATUS"
        )?;
        for (local_id, formation) in self.inner.iter() {
            writeln!(
                tw,
                "{local_id}\t{}\t{}\t{}\t{}",
                formation.name.as_deref().unwrap_or_default(),
                formation.configurations.len(),
                formation.local_status(),
                formation.remote_status(),
            )?;
        }
        tw.flush()?;

        cli_println!(
            "{}",
            String::from_utf8_lossy(
                &tw.into_inner()
                    .map_err(|_| CliError::bail("IO flush error"))?
            )
        );

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Formation {
    name: Option<String>,
    deployed: bool,
    configurations: HashMap<String, FormationConfiguration>,
}

impl Formation {
    fn local_status(&self) -> &str {
        if self.deployed {
            "Deployed"
        } else {
            ""
        }
    }

    fn remote_status(&self) -> &str {
        if self.deployed {
            if self.configurations.values().any(|fc| fc.is_active()) {
                "In Air"
            } else {
                "Grounded"
            }
        } else {
            ""
        }
    }
}
