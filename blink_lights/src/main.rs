use gpio_cdev::{Chip, LineRequestFlags};

fn main() -> Result<(), gpio_cdev::Error> {
    // Read the state of GPIO4 on a raspberry pi.  /dev/gpiochip0
    // maps to the driver for the SoC (builtin) GPIO controller.
    // The LineHandle returned by request must be assigned to a
    // variable (in this case the variable handle) to ensure that
    // the corresponding file descriptor is not closed.
    let mut chip = Chip::new("/dev/gpioc0")?;
    let handle = chip
        .get_line(5)?
        .request(LineRequestFlags::INPUT, 0, "read-input")?;
    for _ in 1..5 {
        println!("Value: {:?}", handle.get_value()?);
    }
    Ok(())
}
