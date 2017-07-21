<?php
/**
 * @file SyntaxCheck.class.php
 * 
 * @author lgh_2002@163.com
 * @version 
 * @desc    提交时校验PHP语法
 * @modify 2015-09-08
 */

require_once dirname(__FILE__) . DIRECTORY_SEPARATOR . 'BasePreCommitCheck.class.php';

class SyntaxCheck extends BasePreCommitCheck 
{
    public function getTitle()
    {
        return "PHP SCRIPT SYNTAX CHECK!!";
    }

    public function renderErrorSummary()
    {
        //return "PHP SCRIPT SYNTAX ERROR FOUND!!";
    }

    public function checkFullFile($lines, $filename)
    {
        $pathinfo = pathinfo($filename);
        $file_suffix = (empty($pathinfo['extension'])) ? '' : $pathinfo['extension'];

        if('php' == $file_suffix) 
        {
            exec("svnlook cat $this->repoName $filename -t $this->trxNum | php -l ", $result);

            if(count($result) <> 1)
            {
                $rs = $result[1] . " ($filename)";
                return $rs;
            }
        }
    }
}


