<?php
/**
 * 默认控制器示例
 */

namespace app\controllers;

use Yaf\Exception;

class Base
{

    /**
     * 检查需要的字段
     * @param $args 参数
     * @param $needs 检查的字段
     * @throws Exception
     */
    public function checkArgsField($args, $need)
    {
        foreach ($need as $v) {
            if (!isset($args[$v]) ){
                $this->throwException("缺少字段:".$v);
            }
        }
        return true;
    }

    /**
     * 过滤需要的数据
     * @param $args 参数
     * @param $needs 需要的字段
     * @return array
     */
    public function filterNeedField($args, $need)
    {
        $tmp = [];
        foreach ($need as $v) {
            if ( isset($args[$v]) && !empty($args[$v]) ){
                $tmp[$v] = $args[$v];
            }
        }
        return $tmp;
    }

    /**
     * 返回GET参数
     * @return mixed
     */
    protected function getQuery()
    {
        return $this->getRequest()->getQuery();
    }

    /**
     * 返回POST参数
     * @return mixed
     * @throws Exception
     */
    protected function getPost()
    {
        $headers = getallheaders();
        if (!empty($headers['Content-Type'])
            && strtolower($headers['Content-Type']) == 'application/json') {
            $data = json_decode(file_get_contents("php://input"), true);
        } else {
            $data = $this->getRequest()->getPost();
        }
        if (!is_array($data)) {
            $this->throwException("参数传递有误");
        }
        return $this->validParams($data);
    }

    protected function validParams($param)
    {
        if (is_array($param)) {
            $R = [];
            foreach ($param as $k => $v) {
                $k = $this->validParams($k);
                $R[$k] = $this->validParams($v);
            }
            return $R;
        } else {
            $farr = array(
                "/<(\\/?)(script|i?frame|style|html|body|title|link|meta|object|\\?|\\%)([^>]*?)>/isU",
                "/(<[^>]*)on[a-zA-Z]+\s*=([^>]*>)/isU",
                "/select\b|insert\b|update\b|delete\b|drop\b|;|`|\"|\'|\/\*|\*|\.\.\/|\.\/|union|into|load_file|outfile|dump/is"
            );
            $param = preg_replace($farr, '', $param);
            return strip_tags($param);
        }
    }

    /**
     * 返回post和get参数
     * @return array
     */
    protected function getParams()
    {
        return array_merge($this->getQuery(), $this->getPost());
    }

    /**
     * @param bool|false $isJson
     * @return mixed|string
     */
    protected function getPhpInput($isJson = false)
    {
        $content = file_get_contents('php://input');
        if ($isJson) {
            $content = \GuzzleHttp\json_decode($content, true);
        }
        return $content;
    }

    /**
     * 抛出异常
     * @param string $message
     * @param int $code
     * @throws Exception
     */
    protected function throwException($message, $code = -2)
    {
        if(is_array($message)){
            $message = json_encode($message);
        }
        throw new Exception($message, $code);
    }
}
