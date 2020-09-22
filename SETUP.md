# VSCode setup

## Add rust targets for cross compilation (mac -> raspi)
Install the needed linker
`brew install arm-linux-gnueabihf-binutils`

Install the arm target for the raspi
`rustup target add armv7-unknown-linux-musleabihf`

# Raspi Setup
## Imaging
Insert the SD card into the Mac & figure out what disk it is.
`diskutil list`

Unmount it.
`diskutil unmountDisk /dev/disk2`

Write a new image to it.
NOTE: `r` in `/dev/rdisk2`
`sudo dd bs=1m if=~/Downloads/2020-05-27-raspios-buster-lite-armhf.img of=/dev/rdisk2; sync`

Eject the disk.
`sudo diskutil eject /dev/rdisk2`

# Raspi Set-up

## mDNS
Install the mDNS daemon
```
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install avahi-daemon
```

Set a new hostname. You'll be able to access the table at `hostname.local`.
```
sudo nano /etc/hosts
sudo nano /etc/hostname
sudo reboot
```

## WiFi
```
$ sudo nano /etc/wpa_supplicant/wpa_supplicant.conf

ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
update_config=1

network = {
    ssid="Network Name"
    psk="Network Password"
    key_mgmt=WPA-PSK
    id_str="Network Nickname"
}
```

## Script
Cross compile the binary `cargo build --release` and copy it over to the raspi.

Add a crontab entry so that the program starts at startup
`sudo crontab -e`

The 0 should be increased depending on if the strip doesn't start inside of the box.
The 300 should be decreased depending on how many LEDs are remaining were removed from the strip.
The 11 should be increased as 300 is decreased. Afaik brightness is non-linear but if `num_leds * 1W < power_supply_watts` then just set this to 31 (max).


So if you remove 50 LEDs from the end, and need to skip 5 at the start then the command would be `19 5 250 1000000 11 25 60`.
```
@reboot /home/pi/infinity-table 19 0 300 1000000 11 25 60
```

## Remove bluetooth

This will speed up the boot process.
`sudo apt-get purge bluez -y && sudo apt-get autoremove -y`

## Read-only
Use `raspi-config` to set the SD card into read-only mode. This should prolong the lifespan of the SD card.
