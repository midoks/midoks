# MySQL 常见问题


```
Got a packet bigger than 'max_allowed_packet' bytes

max_allowed_packet = 20M
set global max_allowed_packet = 2*1024*1024*10
```