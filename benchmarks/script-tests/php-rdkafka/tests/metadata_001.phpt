--TEST--
Metadata 001
--FILE--
<?php

// Metadata can not be in instantiated with `new`
try {
    $m = new RdKafka\Metadata();
} catch (Error $e) {
    echo $e->getMessage(), "\n";
}

// Metadata can be sub-classed and instantiated for the purpose of mocking
class M extends RdKafka\Metadata
{
    public function __construct()
    {
    }
}

$m = new M();
var_dump($m);

// Metadata can be instantied via reflection
$m = (new ReflectionClass(RdKafka\Metadata::class))->newInstanceWithoutConstructor();
var_dump($m);
try {
    $m->getTopics();
} catch(Exception $e) {
    echo $e->getMessage(), "\n";
}
?>
==DONE==
--EXPECTF--
Call to private RdKafka\Metadata::__construct() from %rglobal scope|invalid context%r
object(M)#%d (0) {
}
object(RdKafka\Metadata)#%d (0) {
}
RdKafka\Metadata::__construct() has not been called
==DONE==
