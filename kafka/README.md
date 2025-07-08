
## Run Kafka server 

Start kafka server using following command:

```docker compose up -d```

UI available on `http://localhost:3040`.

(1) source - [Medium post for dev Kafka from _landoop_](https://medium.com/@clasikas/kafka-in-docker-container-and-command-line-67bb0eb2d)

## Produce message on Kafka

```
docker container exec -it kafka /bin/bash
kafka-console-producer --broker-list localhost:9092 --topic test-topic
```

Then publish message e.g.:
```json
{
  "id": "test-id",
  "name": "test-name"
}
```


## Consume message from Kafka

```
docker container exec -it kafka /bin/bash
kafka-console-consumer --bootstrap-server localhost:9092 --topic test-topic -from-beginning
```

## Bonus 

### Create kafka topic with custom partitions
```
kafka-topics --bootstrap-server localhost:9092 --topic <YOUR_TOPIC> --create --partitions <NUMBER>
```

### List of kafka topics
```
kafka-topics --bootstrap-server localhost:9092 --list
kafka-topics.sh --bootstrap-server localhost:9092 --describe
```

### Delete kafka topic
```
kafka-topics --bootstrap-server localhost:9092 --delete --topic <YOUR_TOPIC>

```
