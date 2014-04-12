<?php
function delsvn($dir) {
	$dh = opendir($dir);
	/** 找出所有".svn“ 的文件夹： */
	while ($file = readdir($dh)) {
		if ($file != "." && $file != "..") {
			$fullpath = $dir . "/" . $file;
			if (is_dir($fullpath)) {
				if ($file == ".svn") {
					delsvndir($fullpath);
				} else {
					delsvn($fullpath);
				}
			}
		}
	}

	closedir($dh);

}
function delsvndir($svndir) {

	/** 先删除目录下的文件： */
	$dh = opendir($svndir);
	while ($file=readdir($dh)) {
		if ($file != "." && $file != "..") {
			$fullpath = $svndir . "/" . $file;
			if (is_dir($fullpath)) {
				delsvndir($fullpath);
			} else {
                unlink($fullpath);
                echo '删除了',$fullpath,'目录',"\r\n";
			}
		}

	}
	closedir($dh);

	/** 删除目录文件夹 */
	if (rmdir($svndir)) {
		return  true;
	} else {
		return false;
	}
}

$dir = dirname(__FILE__);
//echo $dir;
delsvn($dir);
?>
