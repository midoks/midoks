#!/bin/bash

# 服务器列表（IP和旧密码）
ips=("1.1.1.1")
passwords=("x")

# SSH端口和用户名
port="22"
user="root"
newpass="xx" # 新密码

# 遍历每个 IP 和对应的密码
for i in ${!ips[@]}; do
    host=${ips[$i]}
    oldpass=${passwords[$i]}

    echo "==== 连接到 $host 修改密码 ===="
    
    # 使用 sshpass 自动输入密码
    sshpass -p "$oldpass" ssh -p $port $user@$host "echo -e \"$newpass\n$newpass\" | passwd $user"

    # 验证新密码
    echo "🔍 验证新密码..."
    sshpass -p "$newpass" ssh -p $port $user@$host "echo '验证成功'"

    if [ $? -eq 0 ]; then
        echo "✅ 新密码验证成功: $host"
    else
        echo "❌ 新密码验证失败: $host"
    fi
done

echo "🎉 所有服务器密码修改完成！"

