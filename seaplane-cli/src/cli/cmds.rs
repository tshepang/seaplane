mod account;
mod completion;
#[cfg(feature = "unstable")]
mod config;
pub mod flight;
pub mod formation;
#[cfg(feature = "unstable")]
mod image;
mod init;
mod license;
pub mod locks;
pub mod metadata;
pub mod restrict;

pub use self::{
    account::SeaplaneAccount,
    completion::SeaplaneShellCompletion,
    flight::SeaplaneFlight,
    formation::{Provider, Region, SeaplaneFormation},
    init::SeaplaneInit,
    license::SeaplaneLicense,
    locks::SeaplaneLocks,
    metadata::SeaplaneMetadata,
    restrict::SeaplaneRestrict,
};
#[cfg(feature = "unstable")]
pub use self::{config::SeaplaneConfig, image::SeaplaneImage};
