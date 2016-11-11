REPOS="$1"
TXN="$2"

# Make sure that the log message contains some text.
SVNLOOK=/usr/bin/svnlook
#$SVNLOOK log -t "$TXN" "$REPOS" | \
#   grep "[a-zA-Z0-9]" > /dev/null || exit 1

# Check that the author of this commit has the rights to perform
# the commit on the files and directories being modified.
#commit-access-control.pl "$REPOS" "$TXN" commit-access-control.cfg || exit 1

LOGMSG=`$SVNLOOK log -t "$TXN" "$REPOS" | grep "[a-zA-Z0-9]" | wc -c`
if [ "$LOGMSG" -lt 5 ];#要求注释不能少于5个字符，您可自定义
then
  echo -e "nLog message cann't be empty! you must input more than 5 chars as comment!." 1>&2
  exit 1
fi 

# All checks passed, so allow the commit.
exit 0
