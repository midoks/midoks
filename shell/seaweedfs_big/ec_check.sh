# /usr/local/bin/seaweedfs-maintenance.sh



tee /tmp/ec_check.sh << 'EOF'
LOCK_FILE="/tmp/seaweedfs-maintenance.lock"
[ -f "$LOCK_FILE" ] && exit 0
touch "$LOCK_FILE"
trap "rm -f $LOCK_FILE" EXIT

weed shell <<'EOF'
lock
ec.rebuild -apply
ec.balance -apply
unlock
EOF

bash /tmp/ec_check.sh

# 计划任务
# */15 * * * * /usr/local/bin/seaweedfs-maintenance.sh >> /var/log/seaweedfs-maintenance.log 2>&1