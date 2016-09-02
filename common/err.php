<?php 

set_error_handler('catchErr');
set_exception_handler('catchErr');

function catchErr(){
	var_dump(func_get_args());
}

$d=1/0;

?>