--TEST--
Metadata\Broker 001
--FILE--
<?php

// Metadata\Broker can not be in instantiated with `new`
try {
    $m = new RdKafka\Metadata\Broker();
} catch (Error $e) {
    echo $e->getMessage(), "\n";
}

// Metadata\Broker can be sub-classed and instantiated for the purpose of mocking
class M extends RdKafka\Metadata\Broker
{
    public function __construct()
    {
    }
}

$i = new M();
var_dump($i);

// Metadata\Broker can be instantied via reflection
$i = (new ReflectionClass(RdKafka\Metadata\Broker::class))->newInstanceWithoutConstructor();
var_dump($i);
try {
    $i->getId();
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
?>
==DONE==
--EXPECTF--
Call to private RdKafka\Metadata\Broker::__construct() from %rglobal scope|invalid context%r
object(M)#%d (0) {
}
object(RdKafka\Metadata\Broker)#%d (0) {
}
RdKafka\Metadata\Broker::__construct() has not been called
==DONE==
