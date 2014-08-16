###
#DATE=$(date +%Y%m%d%s)
#echo $DATE
#MIMA=`echo "M"$DATE | base64 -i`
#MIMA=`cat /dev/urandom | head -n 10 | md5sum | head -c 10`
###
#通过随机数加密
MIMA=`cat /dev/urandom | head -n 10 | base64 | head -c 10`
echo $MIMA
echo "midoks pptpd $MIMA *" > /etc/ppp/chap-secrets
echo $MIMA > /web/vpn/mm.txt
#pkill -9 pptpd
/etc/init.d/pptpd restart-kill
#sleep 1
/etc/init.d/pptpd restart
/etc/init.d/pptpd start
###
# 加入计划任务中
# crontab -l
# crontab -e
# 00 * * * * /bin/sh /web/vpn/cron.sh  #每小时执行一次
