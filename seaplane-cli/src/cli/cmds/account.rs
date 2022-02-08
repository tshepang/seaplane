use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct SeaplaneAccountArgs {
    // subcommands
    #[clap(subcommand)]
    cmd: SeaplaneAccountCmds,
}

#[derive(Subcommand)]
pub enum SeaplaneAccountCmds {
    /// Authenticate a Seaplane account
    Login(SeaplaneAccountLoginArgs),
    /// Logout of a Seaplane account
    Logout(SeaplaneAccountLoginArgs),
}

#[derive(Parser)]
pub struct SeaplaneAccountLoginArgs;

#[derive(Parser)]
pub struct SeaplaneAccountLogoutArgs;
