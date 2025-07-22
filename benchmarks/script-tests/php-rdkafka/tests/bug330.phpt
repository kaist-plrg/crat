--TEST--
TopicPartition destruct for high level consumer
--FILE--
<?php
use RdKafka\Conf;
use RdKafka\KafkaConsumer;

$conf = new Conf();
$conf->set('group.id','test');
$consumer = new KafkaConsumer($conf);
$topic = $consumer->newTopic('test');
unset($topic);
var_dump(isset($topic));
--EXPECT--
bool(false)
