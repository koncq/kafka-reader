use crate::kafka_reader_args::ReadArgs;
use crate::profile::ProfileData;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use serde::Deserialize;

pub fn read_data(profiles: &Vec<ProfileData>, read_args: &ReadArgs) {
    println!("command: Read");
    println!("{:#?}", read_args);
    println!("{:#?}", profiles);

    let profile = if let Some(p) = read_args.profile.as_deref() {
        let xxx = profiles.iter().find(|x| x.get_profile_name().eq(p));
        xxx
    } else {
        None
    };

    let mut topic = "";
    let mut broker = "";
    if !profile.is_none() {
        topic = profile.unwrap().get_topic();
        broker = profile.unwrap().get_broker();
    }

    if read_args.topic.is_some() {
        topic = read_args.topic.as_deref().unwrap();
    }

    if read_args.broker.is_some() {
        broker = read_args.broker.as_deref().unwrap();
    }

    dbg!(topic);
    dbg!(broker);

    let mut consumer = Consumer::from_hosts(vec![broker.to_owned()])
        .with_topic_partitions(topic.to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Latest)
        .with_group("my-group".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", m);

                match str::from_utf8(m.value) {
                    Ok(v) => println!("{:?}", v),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}

#[derive(Debug, Deserialize)]
struct Thing {
    name: String,
    email: String,
}