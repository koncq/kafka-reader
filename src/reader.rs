use crate::arguments::ReadArgs;
use crate::profile::ProfileData;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

pub fn read_data(profiles: &Vec<ProfileData>, read_args: &ReadArgs) {
    println!("command: Read");
    println!("{:#?}", read_args);
    println!("{:#?}", profiles);

    let mode = DecodeMode::Utf8;

    let profile = if let Some(p) = read_args.profile.as_deref() {
        profiles.iter().find(|x| x.get_profile_name().eq(p))
    } else {
        None
    };

    let topic = Property::new(read_args.topic.as_deref(), profile.map(|x| x.get_topic()));
    let broker = Property::new(read_args.broker.as_deref(), profile.map(|x| x.get_broker()));

    println!("topic: {:#?}", topic);
    println!("broker: {:#?}", broker);

    let mut consumer = Consumer::from_hosts(vec![broker.value.to_owned()])
        .with_topic_partitions(topic.value.to_owned(), &[0])
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
                    Ok(v) => match mode {
                        DecodeMode::Utf8 => println!("{}", v),
                        DecodeMode::Proto => todo!(),
                    },
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}

enum DecodeMode {
    Utf8,
    Proto,
}

#[derive(Debug)]
struct Property<T> {
    value: T,
}

impl<T: std::default::Default> Property<T> {
    fn new(value: Option<T>, profile_value: Option<T>) -> Self {
        Self {
            value: value.unwrap_or(profile_value.unwrap_or_default()),
        }
    }
}
