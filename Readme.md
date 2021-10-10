# RPI3 Rust Projects

## blink lights

Simple, 'get your feet on the ground' project to blink lights using rust and rpi3 gpio interface.

### Step One

- [ ] Turn light on, wait, turn light off. Hard code pin and timer.
- [ ] Loop turning light on, wait, turning light off. Hard code pin  and timer. 
- [ ] Loop turning light on, wait, turning light off. Hard code pin  and timer.  Accept cli input for how many times to blink light. Limit 1 to 100. 
- [ ] Loop turning light on, wait, turning light off. Hard code pin .  Accept cli input for how many times to blink light. Limit 1 to 100. Accept cli input for delay in seconds. Limit 1 to 5 seconds.

I tried to use crate rust_gpiozero with FreeBSD and NetBSD, but I got build errors. I tried gpio-cdev with FreeBSD and NetBSD. 
Both cargo check and cargo build worked, but running the compiled program returned and error. I **just** wanted to learn Rust
 on the rpi3, so I install Ubuntu. Now, everything works! Including rustup!

