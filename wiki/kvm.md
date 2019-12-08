## KVM

- mac
```
brew tap jeffreywildman/homebrew-virt-manager
brew install virt-manager virt-viewer
virt-manager -c test:///default
```

```
virt-manager -c 'qemu+ssh://root@192.168.6.10/system?socket=/var/run/libvirt/libvirt-sock'
virt-viewer -c 'qemu+ssh://root@192.168.6.10/system?socket=/var/run/libvirt/libvirt-sock'
```