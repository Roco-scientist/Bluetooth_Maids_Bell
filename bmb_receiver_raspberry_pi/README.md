# Bluetooth Maids Buzzer receiver for raspberry pi 0 w
## Required
Raspberry pi 0 W: <a href=https://www.adafruit.com/product/3400>Buy here</a><rb>
Rust needs to be installed to build<br>

## Bluetooth connection
Check status<br>
`service bluetooth status`<br>
Turn on if it is not on<br>
`service bluetooth start`

### Setup to connect on boot
First edit Bluetooth service in `/etc/systemd/system/dbus-org.bluez.service`<br>
Add the following lines<br>
```
ExecStart=/usr/lib/bluetooth/bluetoothd -C --noplugin=sap
ExecStartPost=/usr/bin/sdptool add SP
```
`sudo systemctl daemon-reload`<br>
`sudo systemctl enable --now bluetooth`<br>
Edit `/etc/modules-load.d/modules.conf` to load rfcomm module automatically<br>
Add the following line<br>
`rfcomm`<br>

### Connect
Find the mac address of the bluetooth from the microcontroller and pair/trust
`sudo bluetoothctl`<br>
```
power on
agent on
scan on
scan off
pair <mac address>
trust <mac address>
quit
```
To bind device:<br>
`sudo rfcomm bind 0 <mac address>`<br>
To bind device on boot, `edit /etc/rc.local` and add:<br>
`rfcomm bind 0 <mac address>`<br>
reboot

## Build and run
Build<br>
`cargo build --release`<br>
Run<br>
`./target/release/bmb_receiver_raspberry_pi`

