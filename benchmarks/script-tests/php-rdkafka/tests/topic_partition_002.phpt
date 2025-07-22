--TEST--
RdKafka\TopicPartition 002
--SKIPIF--
<?php
if (!class_exists('RdKafka\TopicPartition')) {
    echo 'skip';
}
?>
--FILE--
<?php

// TopicPartition can be sub-classed and instantiated for the purpose of mocking
class M extends RdKafka\TopicPartition
{
    public function __construct()
    {
    }
}

$i = new M();
var_dump($i);

// TopicPartition can be instantied via reflection
$i = (new ReflectionClass(RdKafka\TopicPartition::class))->newInstanceWithoutConstructor();
var_dump($i);
try {
    $i->getTopic();
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
?>
==DONE==
--EXPECTF--
object(M)#%d (0) {
}
object(RdKafka\TopicPartition)#%d (0) {
}
RdKafka\TopicPartition::__construct() has not been called
==DONE==
