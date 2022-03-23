mod account;
mod completion;
#[cfg(feature = "unstable")]
mod config;
pub mod flight;
pub mod formation;
#[cfg(feature = "unstable")]
mod image;
mod init;
pub mod kv;
mod license;

pub use self::{
    account::SeaplaneAccount,
    completion::SeaplaneShellCompletion,
    flight::SeaplaneFlight,
    formation::{Provider, Region, SeaplaneFormation},
    init::SeaplaneInit,
    kv::SeaplaneKv,
    license::SeaplaneLicense,
};
#[cfg(feature = "unstable")]
pub use self::{config::SeaplaneConfig, image::SeaplaneImage};
