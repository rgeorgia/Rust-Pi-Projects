use rust_gpiozero::*;
use signal_hook::flag;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

fn main() -> Result<(), Error> {

    // Start using signal_hook. 
    // Example from https://dev.to/talzvon/handling-unix-kill-signals-in-rust-55g6

    let term = Arc::new(AtomicBool::new(false));

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGINT for ctl-c kill signal
    flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    let led = LED::new(18);

    while !term.load(Ordering::Relaxed) {
        led.off();
        println!("OFF");
        thread::sleep(time::Duration::from_secs(1));
        led.on();
        println!("ON");
        thread::sleep(time::Duration::from_secs(3));
    }

    // Since our loop is basically an infinite loop, that only ends when we receive SIGTERM, if
    // we got here, it's because the loop exited after receiving SIGTERM
    println!("Received SIGTERM kill signal. LED is {}", led.value());
    if led.value() {
        println!("Clean up... turning all LEDs to off") ;
        led.off() ;
    }


    Ok(())
}
