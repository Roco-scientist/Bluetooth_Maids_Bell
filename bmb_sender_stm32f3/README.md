# STM32F3DISCOVERY Sender
After a button is pressed, this program sends a signal to the receiver program running on a different controller

## Required
<ul>
<li>STM32F3DISCOVERY board: <a href=https://www.st.com/en/evaluation-tools/stm32f3discovery.html#sample-buy>buy here</a></li>
<li>HC-05 bluetooth board: <a href=https://www.amazon.com/s?k=hc-05&ref=nb_sb_noss_2>buy at Amazon</a> or <a href=https://www.aliexpress.com/item/32340945238.html>buy at Aliexpress China</a></li>
<li>Momentary swith/button</li>
<li>Wires</li>
<li>Resistors</li>
</ul>

## Connection

### Momentary switch
One end connected to 3.3V power supply
The other end connected to PB2 with a 500ohm resister


### HC-05
<table>
<tr>
<th>HC-05 pin</th>
<th>STM32 pin</th>
</tr>
<tr>
<td>5V</td>
<td>5V</td>
</tr>
<tr>
<td>Gnd</td>
<td>Gnd</td>
</tr>
<tr>
<td>RX</td>
<td>PA9</td>
</tr>
<tr>
<td>TX</td>
<td>PA10</td>
</tr>
</table>
