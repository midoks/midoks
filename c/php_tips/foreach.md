### 扩展开发foreach
```c


HashTable *ht = Z_ARRVAL_P(zv);
zend_string *key;
zval *val;
ZEND_HASH_FOREACH_STR_KEY_VAL(ht, key, val){

	zend_hash_del(ht,key);

}ZEND_HASH_FOREACH_END();

```