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

## Requirements
<ul>
<li>Raspberry pi 0 w</li>
<li>STM32F3DISCOVERY</li>
<li>2x STM32F401 black pill (likely overkill and will work with blue pill STM32F103)</li>
<li>2x HC-05 bluetooth boards (1x HC-05 and 1x HC-06 will work too)</li>
<li>Momentary switch/button</li>
<li>Misc electronics; wires, soldering gun, resistors etc.</li>
</ul>

### Only two microcontrolles/raspberry pi's will be needed to get this to work
