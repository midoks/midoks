<?php 

set_error_handler('catchErr');
set_exception_handler('catchErr');

function catchErr(){
    $list = func_get_args();
    array_pop($list);
    var_dump($list);
}

$d=1/0;

?>