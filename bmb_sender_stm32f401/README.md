# sender stm32f401
## Required
<ul>
<li>$3 STM32F401 microcontroller: <a href=https://www.aliexpress.com/item/4001113377360.html>Buy here</a></li>
<li>ST-link v2 STM programmer: <a href=https://www.aliexpress.com/item/32790611727.html>Buy here</a></li>
<li>HC-05 (master/slave) or HC-06 (slave only) bluetooth controller: <a href=https://www.aliexpress.com/item/32340945238.html>Buy here</a></li>
<li>CH340G USB to Serial programmer for bluetooth controller: <a href=https://www.aliexpress.com/item/32809304504.html>Buy here</a></li>
<li>Working openocd and gdb-multiarch for flashing</li>
</ul>

## Setup

### Connection
STM32F401 to HC-05 or HC-06 bluetooth module<br>
<table>
<tr> <th>STM32</th> <th>HC-05/06</th> </tr>
<tr> <td>TX (A9)</td> <td>RX</td> </tr>
<tr> <td>RX (A10)</td> <td>TX</td> </tr>
<tr> <td>5V</td> <td>5V</td> </tr>
<tr> <td>Gnd</td> <td>Gnd</td> </tr>
</table>
<br>
<br>
STM32501 to momentary switch:<br>
Connect STM32 A4 to one end of the switch and 3.3V to the other end<br>

### Build and flash
Download tool chain for cross compilation:<br>
`$ rustup target add thumbv7em-none-eabihf`<br>
Build it:<br>
`$ cargo build --target thumbv7em-none-eabihf --release`<br>
Connect ST-link v2 to STM32F401. Connect ST-link USB to the computer.  Setup openocd in a separate terminal:<br>
`$ openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg`<br>
Connect to STM32F401 through gdb:<br>
```
$ gdb-multiarch -q target/thumbv7em-none-eabihf/release/bmb_sender_stm32f401
(gdb) target remote :3333
(gdb) load
```

### Test
Use slave bluetooth device for testing<br>
In computer terminal connect to device:<br>
```
$ sudo bluetoothctl
[bluetooth]# scan on
```
When device is seen:<br>
```
[bluetooth]# scan off
[bluetooth]# pair <mac address>
[bluetooth]# trust <mac address>
[bluetooth]# quit
$ sudo rfcomm bind 0 <mac address>
$ sudo minicom -D /dev/rfcomm0 -b 115200
```
`<ctrl-a> e`<br>
Press the button and `buzz` should appear within the minicom terminal

