--TEST--
Metadata\Collection 001
--FILE--
<?php

// Metadata\Collection can not be in instantiated with `new`
try {
    new RdKafka\Metadata\Collection();
} catch (Error $e) {
    echo $e->getMessage(), "\n";
}

// Metadata\Collection can be sub-classed and instantiated for the purpose of mocking
class M extends RdKafka\Metadata\Collection
{
    public function __construct()
    {
    }
}

$i = new M();
var_dump($i);

// Metadata\Collection can be instantied via reflection
$i = (new ReflectionClass(RdKafka\Metadata\Collection::class))->newInstanceWithoutConstructor();
var_dump($i);
try {
    $i->count();
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
try {
    foreach ($i as $_) {
    }
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
?>
==DONE==
--EXPECTF--
Call to private RdKafka\Metadata\Collection::__construct() from %rglobal scope|invalid context%r
object(M)#%d (0) {
}
object(RdKafka\Metadata\Collection)#%d (0) {
}
RdKafka\Metadata\Collection::__construct() has not been called
RdKafka\Metadata\Collection::__construct() has not been called
==DONE==
