## Bluetooth connection
Check status<br>
`service bluetooth status`<br>
Turn on if it is not on<br>
`service bluetooth start`

### Connect
`sudo bluetoothctl`<br>
`power on`<br>
`agent on`<br>
`scan on`<br>
`scan off`<br>
`pair <dev>`<br>
`quit`<br>

### Setup to connect on boot
`sudo rfcomm bind hci0 <dev>`
