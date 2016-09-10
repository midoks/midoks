<?php
/**
 *	flush_commands 刷新命令次数
 *	last_query_cost 用查询优化器计算的最后编译的查询的总成本。用于对比同一查询的不同查询方案的成本。默认值0表示还没有编译查询。默认值是0。Last_query_cost具有会话范围。 
 *		开销来自对统计值的估计，但不考虑缓存因素
 *  	5.两种优化方案：
 *		1).静态优化，简单的探测解析树，例如通过代数化法则把WHERE子句转换成相等的形式，静态优化和值无关，即使用不同的参数重新执行查询也不会改变。
 *		2).动态优化，根据上下文而定，和很多因素有关，如where子句中的值和索引中的行数。
 *		MySQL只进行一次静态优化，每次查询时都会进行动态优化
 *	
 *	slow_queries    慢查询次数
 */

include 'DB.php';
$o = new DB();

$flush_conn = $o->squery("show global status like 'flush_commands'");
$data = mysql_fetch_assoc($flush_conn);
//刷新命令执行次数
echo $data['Variable_name'].'命令执行次数:'.$data['Value'];

echo '<br>';

//慢查询次数 | 需要设置
$slow_conn = $o->squery("show global status like 'show_queries'");
$slow_data = mysql_fetch_assoc($slow_conn);
//var_dump($slow_data);
//慢查询执行次数
echo $slow_data['Variable_name'].'命令执行次数:'.$slow_data['Value'];

echo '<br>';


//慢查询次数 | 需要设置
$last_conn = $o->squery("show global status like 'last_query_cost'");
$last_data = mysql_fetch_assoc($last_conn);
//var_dump($last_data);
//慢查询执行次数
echo $last_data['Variable_name'].'命令执行次数:'.$last_data['Value'];



?>
