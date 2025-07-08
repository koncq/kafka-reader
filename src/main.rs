#[macro_use]
extern crate prettytable;
use crate::arguments::read::ReaderArgs;
use crate::arguments::{Commands, ProfileSubcommands};
use crate::configuration::get_profiles;
use crate::profile::{check_if_already_exists, list_profiles};
use crate::reader::read_data;
use clap::Parser;
use color_eyre::Result;
use profile::{ProfileData, add_profile};

mod arguments;
mod configuration;
mod profile;
mod reader;

fn main() -> Result<()> {
    let mut profiles = get_profiles()?;
    let cli = ReaderArgs::parse();

    match &cli.command {
        Some(Commands::Read(read_args)) => read_data(&profiles.profiles, read_args),
        Some(Commands::Profile(profile_args)) => match &profile_args {
            ProfileSubcommands::List => list_profiles(&profiles.profiles),
            ProfileSubcommands::Add(add_args) => {
                if check_if_already_exists(&profiles, &add_args.profile) {
                    panic!("profile already exists");
                }

                add_profile(
                    &mut profiles,
                    ProfileData::new(&add_args.profile, &add_args.topic, &add_args.broker),
                );
            }
            ProfileSubcommands::Edit => {
                todo!("EDITING THE PROFILES")
            }
            ProfileSubcommands::Remove => {
                todo!("REMOVING THE PROFILES")
            }
        },
        _ => println!("command: {:?}", cli.command),
    }

    Ok(())
}
