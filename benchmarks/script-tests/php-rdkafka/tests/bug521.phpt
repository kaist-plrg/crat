--TEST--
Bug #521
--SKIPIF--
<?php version_compare(PHP_VERSION, "8.1") < 0 && die("skip PHP < 8.1"); ?>
--FILE--
<?php

$reflection = new ReflectionMethod(RdKafka\KafkaConsumer::class, 'getMetadata');

foreach ($reflection->getParameters() as $reflectionParam) {
    printf(
        "%s%s%s\n",
        (string) $reflectionParam->getType(),
        $reflectionParam->getType() !== null ? ' ' : '',
        $reflectionParam->getName(),
    );
}
--EXPECT--
bool all_topics
?RdKafka\Topic only_topic
int timeout_ms
