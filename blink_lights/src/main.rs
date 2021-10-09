use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let led = LED::new(18);

    loop{
        led.on();
        println!("ON") ;
        //delay
        sleep(Duration::from_secs(1));
        led.off();
        println!("OFF") ;
        //delay
        sleep(Duration::from_secs(1));
    }
}
