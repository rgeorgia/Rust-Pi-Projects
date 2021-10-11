use rust_gpiozero::*;
use signal_hook::flag;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "blink_lights", about = "Blink light on my rpi3")]
struct Cli {
    #[structopt(short = "b", long = "blinks")]
    number_of_blinks: u64,
    #[structopt(short = "p", long = "pin")]
    pin_number: u8,
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    println!("{:?}", args);

    // Start using signal_hook.
    let term = Arc::new(AtomicBool::new(false));

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGINT for ctl-c kill signal
    flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    let led = LED::new(args.pin_number);

    //while !term.load(Ordering::Relaxed) {
    for n in 0..args.number_of_blinks {
        if term.load(Ordering::Relaxed) {
            break ;
        }
        led.off();
        print!("{}) OFF ", n + 1);
        thread::sleep(time::Duration::from_secs(1));
        led.on();
        println!("ON");
        thread::sleep(time::Duration::from_secs(1));
    }
    //}

    // Since our loop is basically an infinite loop, that only ends when we receive SIGINT, if
    // we got here, it's because the loop exited after receiving SIGINT
    println!("Received SIGINT kill signal. LED is {}", led.value());
    if led.is_lit() {
        println!("Clean up... turning all LEDs to off");
        led.off();
    }

    Ok(())
}
