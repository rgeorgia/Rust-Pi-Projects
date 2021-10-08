# RPI3 Rust Projects

## blink lights

Simple, 'get your feet on the ground' project to blink lights using rust and rpi3 gpio interface.

### Step One

- [ ] Turn light on, wait, turn light off. Hard code pin number and timer.
- [ ] Loop turning light on, wait, turning light off. Hard code pin number and timer. 
- [ ] Loop turning light on, wait, turning light off. Hard code pin number and timer.  Accept cli input for how many times to blink light. Limit 1 to 100. 
- [ ] Loop turning light on, wait, turning light off. Hard code pin number.  Accept cli input for how many times to blink light. Limit 1 to 100. Accept cli input for delay in seconds. Limit 1 to 5 seconds.

To print out GPIO info try:

```bash
sudo gpioctl -f /dev/gpioc0 -l

pin 00:	1	pin 0<IN>
pin 01:	1	pin 1<IN>
pin 02:	1	pin 2<>
pin 03:	1	pin 3<>
pin 04:	1	pin 4<IN>
pin 05:	1	pin 5<IN>
pin 06:	1	pin 6<IN>
pin 07:	0	pin 7<OUT>
pin 08:	0	pin 8<OUT>
pin 09:	0	pin 9<>
pin 10:	0	pin 10<>
pin 11:	0	pin 11<>
pin 12:	0	pin 12<IN>
pin 13:	0	pin 13<IN>
pin 14:	1	pin 14<>
pin 15:	1	pin 15<>
pin 16:	0	pin 16<IN>
pin 17:	0	pin 17<IN>
pin 18:	0	pin 18<IN>
pin 19:	0	pin 19<IN>
pin 20:	0	pin 20<IN>
pin 21:	0	pin 21<IN>
pin 22:	0	pin 22<IN>
pin 23:	0	pin 23<IN>
pin 24:	0	pin 24<IN>
pin 25:	0	pin 25<IN>
pin 26:	0	pin 26<IN>
pin 27:	0	pin 27<IN>
pin 28:	1	pin 28<IN>
pin 29:	0	pin 29<OUT>
pin 30:	1	pin 30<IN>
pin 31:	1	pin 31<IN>
pin 32:	0	pin 32<IN>
pin 33:	0	pin 33<IN>
pin 34:	1	pin 34<IN>
pin 35:	1	pin 35<IN>
pin 36:	1	pin 36<IN>
pin 37:	1	pin 37<IN>
pin 38:	1	pin 38<IN>
pin 39:	1	pin 39<IN>
pin 40:	0	pin 40<>
pin 41:	0	pin 41<>
pin 42:	0	pin 42<>
pin 43:	0	pin 43<>
pin 44:	1	pin 44<IN>
pin 45:	1	pin 45<IN>
pin 46:	1	pin 46<IN>
pin 47:	1	pin 47<OUT>
pin 48:	0	pin 48<>
pin 49:	1	pin 49<PU>
pin 50:	1	pin 50<PU>
pin 51:	1	pin 51<PU>
pin 52:	1	pin 52<PU>
pin 53:	1	pin 53<PU>

```
