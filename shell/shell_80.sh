#!/bin/sh
# midoks

function_cpu_used_rate()
{
    ##echo user nice system idle iowait irq softirq
    CPULOG_1=$(cat /proc/stat | grep 'cpu ' | awk '{print $2" "$3" "$4" "$5" "$6" "$7" "$8}')
    SYS_IDLE_1=$(echo $CPULOG_1 | awk '{print $4}')
    Total_1=$(echo $CPULOG_1 | awk '{print $1+$2+$3+$4+$5+$6+$7}')
    
    sleep 2
    
    CPULOG_2=$(cat /proc/stat | grep 'cpu ' | awk '{print $2" "$3" "$4" "$5" "$6" "$7" "$8}')
    SYS_IDLE_2=$(echo $CPULOG_2 | awk '{print $4}')
    Total_2=$(echo $CPULOG_2 | awk '{print $1+$2+$3+$4+$5+$6+$7}')
    
    SYS_IDLE=`expr $SYS_IDLE_2 - $SYS_IDLE_1`
    
    Total=`expr $Total_2 - $Total_1`
    SYS_USAGE=`expr $SYS_IDLE/$Total*100 |bc -l`
    #echo $SYS_USAGE    
    SYS_Rate=`expr 100-$SYS_USAGE |bc -l`
    Disp_SYS_Rate=`expr "scale=3; $SYS_Rate/1" |bc`
    echo $Disp_SYS_Rate
}

CPU_STATUS=-1
cnt=0
TIME=0
while true ; do
  CPU_STATUS=`function_cpu_used_rate`
  # echo $CPU_STATUS
  if [ $(echo "$CPU_STATUS > 98"|bc) = 1 ];then
    cnt=`expr $cnt + 1`
  else
    cnt=0
  fi
  # echo $cnt
  if [ $cnt -gt 2 ]; then
    echo "overload and running reboot!"
    echo "===="
    service solr restart
    break
  fi

  if [ $TIME -gt 9 ]; then
    echo "running ok!"
    echo "===="
    break
  fi
  sleep 1
  TIME=`expr $TIME + 1`
done
