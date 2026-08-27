#!/bin/bash

curl -O https://rustfs.com/install_rustfs.sh && bash install_rustfs.sh


# Please change the default value for RUSTFS_ACCESS_KEY/RUSTFS_SECRET_KEY immediately and the value can't be rustfsadmin!
# Config file: /etc/default/rustfs

# To change them, run:
#   1. Edit /etc/default/rustfs and update RUSTFS_ACCESS_KEY/RUSTFS_SECRET_KEY
#   2. systemctl restart rustfs

# /etc/systemd/system/rustfs.service