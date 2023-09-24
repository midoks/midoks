#!/bin/bash

# linux图片优化
apt-get install jpegoptim
apt-get install optipng


# find ./uploads/ *.jpg | xargs jpegoptim --size=50k

find ./ *.png | xargs optipng


# optipng -zw 32k uploads/vod/0/0/00bd77d51f9e52ceb9dedec0a77276f5.png
./vod/8/8/88f20782cd8091dbdcb7ae5061b8e7f3.jpg


# jpegoptim --size=50k linuxmi.jpg
# jpegoptim --size=50k uploads/vod/8/e/8e94f0812ff49c35d34ca7bf719d3771.jpg





# cd uploads/vod/0/5/057ed295aaf4634dcdc58aedea5e53ab.jpg

# ls -l uploads/vod/0/5/057ed295aaf4634dcdc58aedea5e53ab.jpg
# -rw-r--r-- 1 root root 11239 Sep 23 20:00 uploads/vod/0/5/057ed295aaf4634dcdc58aedea5e53ab.jpg

# jpegoptim uploads/vod/0/5/057ed295aaf4634dcdc58aedea5e53ab.jpg

# ls -l uploads/vod/0/5/057ed295aaf4634dcdc58aedea5e53ab.jpg
# -rw-r--r-- 1 root root 11235 Sep 24 04:08 uploads/vod/0/5/057ed295aaf4634dcdc58aedea5e53ab.jpg







