--TEST--
RdKafka\Conf::setDefaultTopicConf()
--SKIPIF--
<?php
if (!method_exists('RdKafka\Conf', 'setDefaultTopicConf') || 7 < PHP_MAJOR_VERSION) {
    echo "skip";
}
?>
--FILE--
<?php

$conf = new RdKafka\Conf();

echo "Setting valid topic conf\n";
$conf->setDefaultTopicConf(new RdKafka\TopicConf());

echo "Setting invalid topic conf\n";
$conf->setDefaultTopicConf($conf);

--EXPECTF--
Setting valid topic conf

Deprecated: Function RdKafka\Conf::setDefaultTopicConf() is deprecated in %s%econf_setDefaultTopicConf.php on line 6
Setting invalid topic conf

Deprecated: Function RdKafka\Conf::setDefaultTopicConf() is deprecated in %s%econf_setDefaultTopicConf.php on line 9

Warning: RdKafka\Conf::setDefaultTopicConf() expects parameter 1 to be RdKafka\TopicConf, object given in %s%econf_setDefaultTopicConf.php on line 9

