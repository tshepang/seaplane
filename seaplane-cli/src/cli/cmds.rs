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
pub mod metadata;

pub use self::{
    account::SeaplaneAccount,
    completion::SeaplaneShellCompletion,
    flight::SeaplaneFlight,
    formation::{Provider, Region, SeaplaneFormation},
    init::SeaplaneInit,
    license::SeaplaneLicense,
    metadata::SeaplaneMetadata,
};
#[cfg(feature = "unstable")]
pub use self::{config::SeaplaneConfig, image::SeaplaneImage};
