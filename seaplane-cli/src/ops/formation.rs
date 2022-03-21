use std::{
    collections::HashSet,
    io::Write,
    path::{Path, PathBuf},
    result::Result as StdResult,
    str::FromStr,
};

use seaplane::api::v1::formations::{
    EndpointKey as EndpointKeyModel, EndpointValue as EndpointValueModel,
    FormationConfiguration as FormationConfigurationModel,
};
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;
use uuid::Uuid;

use crate::{
    cli::validator::validate_name,
    context::Ctx,
    error::{CliError, Result},
    fs::{FromDisk, ToDisk},
    ops::Id,
    printer::Output,
};

// TODO: Change out the Vecs for HashMaps where the key is an ID
/// This struct represents a Local Formation. I.e. one the user can interact with on the CLI and can
/// be (de)serialized locally.
///
/// A somewhat counter-intuitive thing about "Formations" and their models is the there is no
/// "Formation Model" only a "Formation Configuration Model" This is because a "Formation" so to
/// speak is really just a named collection of configurations and info about their traffic
/// weights/activation statuses.
#[derive(Debug, Deserialize, Serialize)]
pub struct Formations {
    // Where was this "DB" loaded from on disk, so we can persist it back later
    #[serde(skip)]
    loaded_from: Option<PathBuf>,

    /// A list of "Formation"s
    #[serde(default)]
    pub formations: Vec<Formation>,

    /// A list of "Formation Configuration"s
    ///
    /// We keep these separate from the Formation themselves because multiple formations can
    /// reference the same configuration.
    #[serde(default)]
    pub configurations: Vec<FormationConfiguration>,
}

impl Formations {
    pub fn get_configuration(&self, id: Id) -> Option<&FormationConfiguration> {
        self.configurations.iter().find(|fc| fc.id == id)
    }

    // TODO: this should go away once we're not working with indices anymore
    pub fn get_formation(&self, idx: usize) -> Option<&Formation> {
        self.formations.get(idx)
    }

    // TODO: this should go away once we're not working with indices anymore
    pub fn get_formation_mut(&mut self, idx: usize) -> Option<&mut Formation> {
        self.formations.get_mut(idx)
    }

    pub fn update_or_create_configuration(
        &mut self,
        name: &str,
        model: FormationConfigurationModel,
        in_air: bool,
        uuid: Uuid,
    ) -> Vec<Id> {
        let mut ret = Vec::new();

        if !self.configurations.iter().any(|c| c.model == model) {
            let mut new_cfg = FormationConfiguration::new(model.clone());
            new_cfg.remote_id = Some(uuid);
            ret.push(new_cfg.id);
            self.configurations.push(new_cfg);
        }

        for cfg in self.configurations.iter().filter(|cfg| cfg.model == model) {
            for f in self
                .formations
                .iter_mut()
                .filter(|f| f.name.as_deref() == Some(name))
            {
                if in_air {
                    f.in_air.insert(cfg.id);
                    f.grounded.remove(&cfg.id);
                } else {
                    f.grounded.insert(cfg.id);
                    f.in_air.remove(&cfg.id);
                }
                f.local.insert(cfg.id);
            }
            ret.push(cfg.id);
        }

        ret
    }

    // TODO: add success indicator
    pub fn add_uuid(&mut self, id: &Id, uuid: Uuid) {
        for cfg in self.configurations.iter_mut() {
            if &cfg.id == id {
                cfg.remote_id = Some(uuid);
                break;
            }
        }
    }

    // TODO: add success indicator
    pub fn add_in_air_by_name(&mut self, name: &str, id: Id) {
        for f in self.formations.iter_mut() {
            if f.name.as_deref() == Some(name) {
                f.in_air.insert(id);
                break;
            }
        }
    }

    // TODO: add success indicator
    pub fn add_grounded_by_name(&mut self, name: &str, id: Id) {
        for f in self.formations.iter_mut() {
            if f.name.as_deref() == Some(name) {
                f.grounded.insert(id);
                f.in_air.remove(&id);
                break;
            }
        }
    }

    /// Returns true if there is a Formation with the given name
    pub fn contains_name(&self, name: &str) -> bool {
        self.formations
            .iter()
            .any(|f| f.name.as_deref() == Some(name))
    }

    /// Removes an exact name match, returning the removed Formation or None if nothing matched.
    pub fn remove_name(&mut self, name: &str) -> Option<Formation> {
        if let Some(idx) = self.formation_index_of_name(name) {
            return Some(self.formations.swap_remove(idx));
        }

        None
    }

    // TODO: this should go away once we're not working with indices anymore
    /// Returns the index of an exact name match
    pub fn formation_index_of_name(&self, name: &str) -> Option<usize> {
        self.formations
            .iter()
            .enumerate()
            .find(|(_, f)| f.name.as_deref() == Some(name))
            .map(|(i, _)| i)
    }

    // TODO: this should go away once we're not working with indices anymore
    /// Returns all indices of an exact name or partial ID match
    pub fn formation_indices_of_matches(&self, name: &str) -> Vec<usize> {
        self.formations
            .iter()
            .enumerate()
            .filter(|(_, f)| f.name.as_deref() == Some(name) || f.id.to_string().starts_with(name))
            .map(|(i, _)| i)
            .collect()
    }

    // TODO: this should go away once we're not working with indices anymore
    /// Returns all indices of a partial name or ID match
    pub fn formation_indices_of_left_matches(&self, name: &str) -> Vec<usize> {
        self.formations
            .iter()
            .enumerate()
            .filter(|(_, f)| {
                f.name
                    .as_deref()
                    .map(|n| n.starts_with(name))
                    .unwrap_or(false)
                    || f.id.to_string().starts_with(name)
            })
            .map(|(i, _)| i)
            .collect()
    }

    // TODO: this should go away once we're not working with indices anymore
    /// Removes all indices
    pub fn remove_formation_indices(&mut self, indices: &[usize]) -> Vec<Formation> {
        // TODO: There is probably a much more performant way to remove a bunch of times from a Vec
        // but we're talking such a small number of items this should never matter.

        indices
            .iter()
            .enumerate()
            .map(|(i, idx)| self.formations.remove(idx - i))
            .collect()
    }
}

impl FromDisk for Formations {
    fn set_loaded_from<P: AsRef<Path>>(&mut self, p: P) {
        self.loaded_from = Some(p.as_ref().into());
    }

    fn loaded_from(&self) -> Option<&Path> {
        self.loaded_from.as_deref()
    }
}

impl ToDisk for Formations {}

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
            "LOCAL ID\tNAME\tLOCAL\tDEPLOYED (GROUNDED)\t DEPLOYED (IN AIR)\t TOTAL CONFIGURATIONS"
        )?;
        for formation in &self.formations {
            let local = formation.local.len();
            let in_air = formation.in_air.len();
            let grounded = formation.grounded.len();
            let total = formation
                .in_air
                .union(
                    &formation
                        .grounded
                        .union(&formation.local)
                        .copied()
                        .collect(),
                )
                .count();

            writeln!(
                tw,
                "{}\t{}\t{}\t{}\t{}\t{}",
                &formation.id.to_string()[..8], // TODO: make sure length is not ambiguous
                formation.name.as_deref().unwrap_or_default(),
                local,
                grounded,
                in_air,
                total
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

// TODO: move ID to the key of a HashMap
#[derive(Debug, Deserialize, Serialize)]
pub struct Formation {
    pub id: Id,
    pub name: Option<String>,
    pub local: HashSet<Id>,
    pub in_air: HashSet<Id>,
    pub grounded: HashSet<Id>,
}

impl Formation {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            id: Id::new(),
            name: Some(name.into()),
            local: HashSet::new(),
            in_air: HashSet::new(),
            grounded: HashSet::new(),
        }
    }

    /// Returns the Formation Configuration IDs that are neither Grounded (Inactive) or In Air (active)
    pub fn local_only_configs(&self) -> Vec<Id> {
        self.local
            .difference(&self.in_air.union(&self.grounded).copied().collect())
            .copied()
            .collect()
    }
}

/// Wraps the [`FormationConfiguration`] model adding a local ID and the UUID associated
#[derive(Debug, Serialize, Deserialize)]
pub struct FormationConfiguration {
    pub id: Id,
    remote_id: Option<Uuid>,
    pub model: FormationConfigurationModel,
}

impl FormationConfiguration {
    pub fn new(model: FormationConfigurationModel) -> Self {
        Self {
            id: Id::new(),
            remote_id: None,
            model,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Endpoint {
    src: EndpointSrc,
    dst: EndpointDst,
}

impl Endpoint {
    pub fn key(&self) -> EndpointKeyModel {
        match &self.src {
            EndpointSrc::Http(p) => EndpointKeyModel::Http { path: p.to_owned() },
            EndpointSrc::Tcp(p) => EndpointKeyModel::Tcp { port: *p },
            EndpointSrc::Udp(p) => EndpointKeyModel::Udp { port: *p },
        }
    }
    pub fn value(&self) -> EndpointValueModel {
        EndpointValueModel {
            flight_name: self.dst.flight.clone(),
            port: self.dst.port,
        }
    }
}

impl FromStr for Endpoint {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split('=');
        Ok(Self {
            src: parts
                .next()
                .ok_or_else(|| String::from("invalid endpoint source"))?
                .parse()?,
            dst: parts
                .next()
                .ok_or_else(|| String::from("invalid endpoint destination"))?
                .parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EndpointSrc {
    Http(String),
    Tcp(u16),
    Udp(u16),
}

impl FromStr for EndpointSrc {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split(':');
        let proto = parts
            .next()
            .ok_or_else(|| String::from("missing endpoint protocol"))?;
        let ep = match &*proto.to_ascii_lowercase() {
            "http" => EndpointSrc::Http(
                parts
                    .next()
                    .ok_or_else(|| String::from("missing network port number"))?
                    .to_string(),
            ),
            "tcp" => EndpointSrc::Tcp(
                parts
                    .next()
                    .ok_or_else(|| String::from("missing network port number"))?
                    .parse::<u16>()
                    .map_err(|_| String::from("invalid network port number"))?,
            ),
            "udp" => EndpointSrc::Udp(
                parts
                    .next()
                    .ok_or_else(|| String::from("missing network port number"))?
                    .parse::<u16>()
                    .map_err(|_| String::from("invalid network port number"))?,
            ),
            _ => {
                return Err(format!(
                    "invalid protocol '{}' (valid options: http, tcp, udp)",
                    proto
                ))
            }
        };
        Ok(ep)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EndpointDst {
    flight: String,
    port: u16,
}

impl FromStr for EndpointDst {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split(':');
        let flight = parts
            .next()
            .ok_or_else(|| ("missing destinaion flight").to_string())?;
        validate_name(flight)?;
        let port = parts
            .next()
            .ok_or_else(|| ("missing destination port number").to_string())?
            .parse::<u16>()
            .map_err(|_| ("invalid port number").to_string())?;

        Ok(Self {
            flight: flight.to_string(),
            port,
        })
    }
}

#[cfg(test)]
mod endpoint_test {
    use super::*;

    #[test]
    fn endpoint_valid_http() {
        let ep: Endpoint = "http:/foo/bar=baz:1234".parse().unwrap();
        assert_eq!(
            ep,
            Endpoint {
                src: EndpointSrc::Http("/foo/bar".into()),
                dst: EndpointDst {
                    flight: "baz".into(),
                    port: 1234
                }
            }
        )
    }

    #[test]
    fn endpoint_missing_dst_or_src() {
        assert!("baz:1234".parse::<Endpoint>().is_err());
    }

    // TODO: might remove and allow to elide
    #[test]
    fn endpoint_missing_http_proto() {
        assert!("1234=baz:1234".parse::<Endpoint>().is_err());
        assert!(":1234=baz:1234".parse::<Endpoint>().is_err());
        assert!("/foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!("/foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!("=baz:1234".parse::<Endpoint>().is_err());
        assert!(":=baz:1234".parse::<Endpoint>().is_err());
    }

    // TODO: might allow eliding destination port
    #[test]
    fn endpoint_missing_dst() {
        assert!("tcp:1234=baz".parse::<Endpoint>().is_err());
        assert!("udp:1234=:1234".parse::<Endpoint>().is_err());
        assert!("http:/foo/bar=baz:".parse::<Endpoint>().is_err());
        assert!("http:/foo/bar=".parse::<Endpoint>().is_err());
    }

    #[test]
    fn endpoint_valid_tcp() {
        let ep: Endpoint = "tcp:1234=baz:4321".parse().unwrap();
        assert_eq!(
            ep,
            Endpoint {
                src: EndpointSrc::Tcp(1234),
                dst: EndpointDst {
                    flight: "baz".into(),
                    port: 4321
                }
            }
        )
    }

    #[test]
    fn endpoint_invalid_tcp_udp() {
        assert!("udp:/foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!("udp:1234=baz:9999999".parse::<Endpoint>().is_err());
        assert!("udp:1234=baz:/foo".parse::<Endpoint>().is_err());
    }
}
