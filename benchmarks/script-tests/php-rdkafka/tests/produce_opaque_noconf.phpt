--TEST--
Produce with opaque, no conf
--SKIPIF--
<?php
require __DIR__ . '/integration-tests-check.php';
RD_KAFKA_BUILD_VERSION < 0x1000000 && die("skip librdkafka < 1.0.0");
RD_KAFKA_BUILD_VERSION >= 0x1050000 && die("skip librdkafka >= 1.5.0");
--FILE--
<?php
require __DIR__ . '/integration-tests-check.php';

$producer = new RdKafka\Producer();
var_dump($producer->addBrokers(getenv('TEST_KAFKA_BROKERS')));

$topicName = sprintf("test_rdkafka_%s", uniqid());

$topic = $producer->newTopic($topicName);

if (!$producer->getMetadata(false, $topic, 10*1000)) {
    echo "Failed to get metadata, is broker down?\n";
}

for ($i = 0; $i < 10; $i++) {
    $topic->produce(0, 0, "message $i", null, "opaque $i");
}

echo "Expect no leaks\n";
--EXPECT--
int(1)
Expect no leaks
