# 解决设置table中td宽度不生效

```
特性：table是一个整体，每一列td的宽度是由一个其中一个最长td的宽度决定的。
解决：一定要在table标签上加word-wrap: break-word; word-break: break-all;之后再设置百分比宽度就可以生效了(获取你用的bootstrap，可以添加col-md-1)
```