#!/usr/bin/bash

# care if you have multiple eth interfaces
eth_device=$(ls /sys/class/net | grep enp | head -n1)
sudo ethtool -h "$eth_device" wol g
connection_name=$(sudo nmcli --fields device,name connection show | grep "$eth_device" | cut -d" " -f2- | xargs)
sudo nmcli connection modify "$connection_name" 802-a-ethernet.wake-on-lan magic