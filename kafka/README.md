
## Run Kafka server 

Start kafka server using following command:

```docker compose up -d```

UI available on `http://localhost:3040`.

## Produce message on Kafka

```
docker container exec -it kafka /bin/bash
kafka-console-producer --broker-list localhost:9092 --topic test-message-in
```


## Consume message from Kafka

```
docker container exec -it kafka /bin/bash
kafka-console-consumer --bootstrap-server localhost:9092 --topic test-message-in -from-beginning
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
