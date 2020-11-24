# receiver stm32f401
### WARNING Still not tested on STM32F401
## Index
## Required
<ul>
<li>$3 STM32F401 microcontroller: <a href=https://www.aliexpress.com/item/4001113377360.html>Buy here</a></li>
<li>ST-link v2 STM programmer: <a href=https://www.aliexpress.com/item/32790611727.html>Buy here</a></li>
<li>HC-05 (master/slave) or HC-06 (slave only) bluetooth controller: <a href=https://www.aliexpress.com/item/32340945238.html>Buy here</a></li>
<li>CH340G USB to Serial programmer for bluetooth controller: <a href=https://www.aliexpress.com/item/32809304504.html>Buy here</a></li>
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
STM32501 to Buzzer:<br>
Connect STM32 B9 to Buzzer + and Ground to buzzer -<br>
