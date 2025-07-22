--TEST--
RdKafka\Conf::setDefaultTopicConf()
--SKIPIF--
<?php
if (!method_exists('RdKafka\Conf', 'setDefaultTopicConf') || 8 > PHP_MAJOR_VERSION) {
    echo "skip";
}
?>
--FILE--
<?php

$conf = new RdKafka\Conf();

echo "Setting valid topic conf\n";
$conf->setDefaultTopicConf(new RdKafka\TopicConf());

echo "Setting invalid topic conf\n";
try {
    $conf->setDefaultTopicConf($conf);
} catch(TypeError $error) {
    echo $error->getMessage() . PHP_EOL;
    echo $error->getFile() . PHP_EOL;
    echo $error->getLine() . PHP_EOL;
    echo $error->getCode();
}

--EXPECTF--
Setting valid topic conf

Deprecated: Method RdKafka\Conf::setDefaultTopicConf() is deprecated in %s%econf_setDefaultTopicConf8.php on line 6
Setting invalid topic conf

Deprecated: Method RdKafka\Conf::setDefaultTopicConf() is deprecated in %s%econf_setDefaultTopicConf8.php on line 10
RdKafka\Conf::setDefaultTopicConf(): Argument #1 ($topic_conf) must be of type RdKafka\TopicConf, RdKafka\Conf given
%s%econf_setDefaultTopicConf8.php
10
0

