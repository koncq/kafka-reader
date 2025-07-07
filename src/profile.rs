use crate::configuration::{ProfilesConfig, save_configuration};
use prettytable::Table;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileData {
    profile: String,
    topic: String,
    broker: String,
}

impl ProfileData {
    pub fn new(profile: &str, topic: &str, broker: &str) -> Self {
        Self {
            profile: profile.to_string(),
            topic: topic.to_string(),
            broker: broker.to_string(),
        }
    }

    pub fn get_profile_name(&self) -> String {
        String::from(&self.profile)
    }

    pub fn get_topic(&self) -> &str {
        &self.topic
    }

    pub fn get_broker(&self) -> &str {
        &self.broker
    }
}

pub fn list_profiles(profiles: &Vec<ProfileData>) {
    let mut table = Table::new();
    println!("Available profiles:");
    table.add_row(row!["Profile name", "Topic name", "Broker address"]);
    for profile in profiles {
        table.add_row(row![
            profile.get_profile_name(),
            profile.get_topic(),
            profile.get_broker()
        ]);
    }
    table.printstd();
}

pub fn add_profile(profiles: &mut ProfilesConfig, profile_data: ProfileData) {
    println!("add profile data: {:#?}", profile_data);

    profiles.profiles.push(profile_data);

    save_configuration(profiles).unwrap();
}

pub fn check_if_already_exists(profiles: &ProfilesConfig, profile_name: &str) -> bool {
    profiles
        .profiles
        .iter()
        .any(|p| p.profile == String::from(profile_name))
}
