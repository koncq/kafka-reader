#[macro_use]
extern crate prettytable;
use crate::configuration::get_profiles;
use crate::arguments::{KafkaReaderArgs, KafkaReaderCommands, ProfileSubcommands};
use crate::profile::{check_if_already_exists, list_profiles};
use crate::reader::read_data;
use clap::Parser;
use color_eyre::Result;
use profile::{ProfileData, add_profile};

mod configuration;
mod arguments;
mod profile;
mod reader;

fn main() -> Result<()> {
    let mut profiles = get_profiles()?;
    let cli = KafkaReaderArgs::parse();

    match &cli.command {
        Some(KafkaReaderCommands::Read(read_args)) => read_data(&profiles.profiles, read_args),
        Some(KafkaReaderCommands::Profile(profile_args)) => match &profile_args {
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
