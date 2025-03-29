
BS_HELP(){
    echo "bash run.sh run|r            -> 策略1"
}

case "$1" in
    "run" | "r") BS_CMD ;;
    *) BS_HELP;;
esac