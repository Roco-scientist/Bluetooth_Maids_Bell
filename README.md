# Bluetooth Maids Buzzer
### Still a work in progress
## Purpose
The puprose of this git repo is to create a bluetooth buzzer that sounds when a button is pressed in a different room.
This will be done by connection two microcontrollers/raspberry pi with a bluetooth protocol, where when a button is pressed on one end
it sends a signal through the bluetooth and a buzzer turns on at the other end.<br>
<ul>
<li>Start with connecting a STM32F3DISCOVERY microcontroller to a Raspberry pi 0 W (I have both in hand)</li>
<li>Transfer to STM32F401 black pill microncontrollers after they arrive</li>
</ul>

## Index
<ul>
<li><a href="#requirements">Requirements</a></li>
<li><a href="#setup">Setup</a></li>
<li></li>
<li></li>
</ul>

## Requirements
<ul>
<li>Raspberry pi 0 w</li>
<li>STM32F3DISCOVERY</li>
<li>2x STM32F401 black pill (likely overkill and will work with blue pill STM32F103)</li>
<li>2x HC-05 bluetooth boards (1x HC-05 and 1x HC-06 will work too)</li>
<li>USB serial converter for programming HC-05: <a href=https://www.aliexpress.com/item/32809304504.html>Available here</a></li>
<li>Momentary switch/button</li>
<li>Misc electronics; wires, soldering gun, resistors etc.</li>
</ul>

### Only two microcontrolles/raspberry pi's will be needed to get this to work

## Setup
### Bluetooth: HC-05 or HC-06 setup
Initial setup
<ol>
<li>Connect HC-05 TX to RX of USB to serial device</li>
<li>Connect HC-05 RX to TX of USB to serial device</li>
<li>Connect HC-05 5v/gnd to USB to serial device</li>
<li>Hold HC-05 button down then connect the USB to a computer device</li>
</ol>

If done correctly, the bluetooth will slowing blink once every two seconds.  It is in AT mode<br>
<br>
Connect to minicom<br>
`sudo mincom -D /dev/ttyUSB0 -b 38400`<br>
Within minicom each command needs an enter, then ctrl-j<br>
Replace `new_name` and `0000` with your choice<br>
Change buad rate to 115200 with 1 stop bit and no parity.  Set AT+ROLE=0 on the receiver bluetooth.  0: slave, 1: master<br>
```
AT
AT+VERSION
AT+ADDR
AT+UART=115200,1,0
AT+NAME=new_name
AT+PSWD="0000"
AT+ROLE=0
```
