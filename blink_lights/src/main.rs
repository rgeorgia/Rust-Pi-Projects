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
    #[structopt(short = "b", long = "blinks", help="Min 1, Max 100")]
    number_of_blinks: u64,
    #[structopt(short = "p", long = "pin")]
    pin_number: u8,
    #[structopt(short = "f", long = "off", default_value = "1", help = "Range from 1 to 5")]
    off_time: u64,
    #[structopt(short = "o", long = "on", default_value = "1", help = "Range from 1 to 5")]
    on_time: u64,
}

const MAX_BLINK: u64 = 100;
const MAX_TIME: u64 = 10 ;

fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    if args.number_of_blinks > MAX_BLINK {
        println!("The number of blinks cannot be greater than {}", MAX_BLINK) ;
        return Ok(()) ;
    }

    if args.off_time > MAX_TIME || args.on_time > MAX_TIME {
        println!("On or off time cannot be greater than {}", MAX_TIME) ;
        return Ok(()) ;
    }


    // Start using signal_hook.
    let term = Arc::new(AtomicBool::new(false));

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGINT for ctl-c kill signal
    flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    let led = LED::new(args.pin_number);

    for n in 0..args.number_of_blinks {
        if term.load(Ordering::Relaxed) {
            break ;
        }
        led.off();
        print!("{}) OFF ", n + 1);
        thread::sleep(time::Duration::from_secs(args.off_time));
        led.on();
        println!("ON");
        thread::sleep(time::Duration::from_secs(args.on_time));
    }

    // Since our loop is basically an infinite loop, that only ends when we receive SIGINT, if
    // we got here, it's because the loop exited after receiving SIGINT
    println!("Received SIGINT kill signal. LED is {}", led.value());
    if led.is_lit() {
        println!("Clean up... turning all LEDs to off");
        led.off();
    }

    Ok(())
}
