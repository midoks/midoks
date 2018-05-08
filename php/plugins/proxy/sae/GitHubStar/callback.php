<?php 

if(!empty($_GET)){
	var_dump($_GET);
}

?>



<script type="text/javascript">
<?php 
	if(!empty($_GET['code'])){

		echo 'var code="',$_GET['code'],'";';
	}
?>


</script>