--TEST--
Metadata\Topic 001
--FILE--
<?php

// Metadata\Topic can not be in instantiated with `new`
try {
    new RdKafka\Metadata\Topic();
} catch (Error $e) {
    echo $e->getMessage(), "\n";
}

// Metadata\Topic can be sub-classed and instantiated for the purpose of mocking
class M extends RdKafka\Metadata\Topic
{
    public function __construct()
    {
    }
}

$i = new M();
var_dump($i);

// Metadata\Topic can be instantied via reflection
$i = (new ReflectionClass(RdKafka\Metadata\Topic::class))->newInstanceWithoutConstructor();
var_dump($i);
try {
    $i->getTopic();
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
?>
==DONE==
--EXPECTF--
Call to private RdKafka\Metadata\Topic::__construct() from %rglobal scope|invalid context%r
object(M)#%d (0) {
}
object(RdKafka\Metadata\Topic)#%d (0) {
}
RdKafka\Metadata\Topic::__construct() has not been called
==DONE==
