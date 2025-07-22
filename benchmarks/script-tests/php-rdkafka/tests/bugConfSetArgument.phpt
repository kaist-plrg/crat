--TEST--
RdKafka\Conf
--SKIPIF--
<?php
version_compare(PHP_VERSION, "7.1") < 0 && die("skip PHP < 7.1");
--FILE--
<?php

class TestBug extends RdKafka\Conf
{
    public function set($name, $value): void
    {
        parent::set($name, $value);
    }
}

$conf = new TestBug();
$conf->set('metadata.broker.list', '127.0.0.1');

echo "done" . PHP_EOL;
--EXPECT--
done
