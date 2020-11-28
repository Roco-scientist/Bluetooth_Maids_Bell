use rppal::{gpio, uart};
use std::{thread, time};

fn main() {
    // create bluetooth uart connection with no parity, 8 bits data, and 1 bit stopping
    // rfcomm0 is the bound address of HC-05 from the microcontroller
    let mut bluetooth =
        uart::Uart::with_path("/dev/rfcomm0", 115200, uart::Parity::None, 8, 1).unwrap();

    // setup the bluetooth to pause waiting for a read with a minimum of 4 bits
    bluetooth
        .set_read_mode(4u8, time::Duration::from_secs(0u64))
        .unwrap();

    // setup blocking write so writing is always completed
    bluetooth.set_write_mode(true).unwrap();

    // Setup buzzer output pin
    let gpio = gpio::Gpio::new().unwrap();
    let mut buzzer_pin = gpio.get(24).unwrap().into_output();

    loop {
        // create empty data array to put read data into
        let mut data = [0; 32usize];

        // Wait for signal to come from sender and put read into data array
        bluetooth.read(&mut data).unwrap();

        // print for debugging
        println!("{:?}", data);

        // return a signal of receipt and confirmation
        // bluetooth.write(&b"RECEIVED"[..]).unwrap();

        // run buzzer signal
        buzz(&mut buzzer_pin, 300, time::Duration::from_millis(500)).unwrap();
        thread::sleep(time::Duration::from_millis(250));
        buzz(&mut buzzer_pin, 1000, time::Duration::from_millis(500)).unwrap();
        thread::sleep(time::Duration::from_millis(250));
        buzz(&mut buzzer_pin, 200, time::Duration::from_millis(500)).unwrap();

        // flush anything else remaining in the buffers
        bluetooth.flush(uart::Queue::Both).unwrap();
    }
}

fn buzz(
    pin: &mut gpio::OutputPin,
    hz: u32,
    duration: time::Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    // start with pin in low to make sure
    pin.set_low();

    // find the number of times signal needs to be changed.  2x because both up and down need to be
    // set
    let repeats = (hz as f32 * duration.as_millis() as f32 / 1000f32) as u32;

    // find the puases needed to crete the duration without a timer
    let pause = duration / repeats;

    // set the pin to high then low with previous paramaters
    for _ in 0..repeats {
        pin.set_high();
        thread::sleep(pause);
        pin.set_low();
        thread::sleep(pause);
    }
    // Return result in case an unwrap or something similar needed later
    Ok(())
}
