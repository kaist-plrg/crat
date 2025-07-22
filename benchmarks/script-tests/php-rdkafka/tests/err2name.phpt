--TEST--
err2name
--FILE--
<?php
var_dump(rd_kafka_err2name(RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE));
--EXPECT--
string(19) "OFFSET_OUT_OF_RANGE"
