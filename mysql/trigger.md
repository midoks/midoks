- 创建触发器

```
drop trigger if exists t_user_pass_change;
create trigger t_user_pass_change after update on uc_members
for each row
begin
  replace  into uc_members_logs(uid, password, update_time) values(new.uid, new.password, unix_timestamp(now()));
end;
```