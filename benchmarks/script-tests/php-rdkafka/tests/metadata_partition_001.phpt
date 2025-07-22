--TEST--
Metadata\Partition 001
--FILE--
<?php

// Metadata\Partition can not be in instantiated with `new`
try {
    new RdKafka\Metadata\Partition();
} catch (Error $e) {
    echo $e->getMessage(), "\n";
}

// Metadata\Partition can be sub-classed and instantiated for the purpose of mocking
class M extends RdKafka\Metadata\Partition
{
    public function __construct()
    {
    }
}

$i = new M();
var_dump($i);

// Metadata\Partition can be instantied via reflection
$i = (new ReflectionClass(RdKafka\Metadata\Partition::class))->newInstanceWithoutConstructor();
var_dump($i);
try {
    $i->getId();
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
?>
==DONE==
--EXPECTF--
Call to private RdKafka\Metadata\Partition::__construct() from %rglobal scope|invalid context%r
object(M)#%d (0) {
}
object(RdKafka\Metadata\Partition)#%d (0) {
}
RdKafka\Metadata\Partition::__construct() has not been called
==DONE==
