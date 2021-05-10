### Linux KVM [虚拟化]


###安装[centos7]

```

yum install -y qemu-kvm libvirt virt-install bridge-utils

lsmod |grep kvm



wget http://mirrors.163.com/centos/7.9.2009/isos/x86_64/CentOS-7-x86_64-Minimal-2009.iso /home/



virt-install \
--virt-type=kvm \
--name=kvm-1 \
--vcpus=1 \
--memory=1024 \
--location=/home/CentOS-7-x86_64-Minimal-2009.iso \
--disk path=/data/vms/kvm-1.qcow2,size=15,format=qcow2 \
--network bridge=virbr0 \
--graphics none \
--extra-args='console=ttyS0' \
--force

```



## 判断出当前环境所使用的虚拟技术
```
wget http://people.redhat.com/~rjones/virt-what/files/virt-what-1.15.tar.gz
tar zxf virt-what-1.15.tar.gz
cd virt-what-1.15/
./configure && make && make install
virt-what
```