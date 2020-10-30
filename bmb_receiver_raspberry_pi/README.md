## Bluetooth connection
Check status<br>
`service bluetooth status`<br>
Turn on if it is not on<br>
`service bluetooth start`

### Setup to connect on boot
First edit Bluetooth service in `/etc/systemd/system/dbus-org.bluez.service`<br>
Add the following lines<br>
```
ExecStart=/usr/lib/bluetooth/bluetoothd -C
ExecStartPost=/usr/bin/sdptool add SP
```
`sudo systemctl daemon-reload`<br>
`sudo systemctl enable --now bluetooth`<br>
Edit `/etc/modules-load.d/modules.conf` to load rfcomm module automatically<br>
Add the following line<br>
`rfcomm`<br>
reboot

### Connect
`sudo bluetoothctl`<br>
`power on`<br>
`agent on`<br>
`scan on`<br>
`scan off`<br>
`pair <dev>`<br>
`trust <dev>`<br>
`quit`<br>

`sudo rfcomm bind rfcomm0 <mac address>`
