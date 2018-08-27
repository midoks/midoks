<?php
// 命名空间
namespace app\library;

class Coordinate
{
    public $x = 0;
    public $y = 0;
    /**
     * Coordinate constructor.
     * @param $lon float 经度
     * @param $lat float 纬度
     */
    public function __construct ($lon,$lat)
    {
        $this->x = $lon;
        $this->y = $lat;
    }
}