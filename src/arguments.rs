pub mod profile;
pub mod read;

pub use crate::arguments::profile::ProfileSubcommands;
pub use crate::arguments::read::ReadArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command()]
    Read(ReadArgs),

    #[command(subcommand)]
    Profile(ProfileSubcommands),
}
