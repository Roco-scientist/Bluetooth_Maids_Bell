use rppal::{gpio, uart};
use std::{thread, time};

fn main() {
    // create bluetooth uart connection with no parity, 8 bits data, and 1 bit stopping
    // rfcomm0 is the bound address of HC-05 from the microcontroller
    let mut bluetooth =
        uart::Uart::with_path("/dev/rfcomm0", 9600, uart::Parity::None, 8, 1).unwrap();

    // setup the bluetooth to pause waiting for a read with a minimum of 4 bits
    bluetooth
        .set_read_mode(4u8, time::Duration::from_secs(0u64))
        .unwrap();

    // setup blocking write so writing is always completed
    bluetooth.set_write_mode(true).unwrap();

    // Setup buzzer output pin
    let gpio = gpio::Gpio::new();
    let buzzer_pin = gpio.get(23).unwrap().into_output();

    loop {
        // create empty data array to put read data into
        let mut data = [0u8; 32usize];

        // Wait for signal to come from sender and put read into data array
        bluetooth.read(&mut data).unwrap();

        // print for debugging
        println!("{:?}", data);

        // return a signal of receipt and confirmation
        bluetooth.write(&b"RECEIVED"[..]).unwrap();

        // set buzzer signal
        buzz(buzzer_pin, 500, time::Duration::from_secs(3)).unwrap();

        // flush anything else remaining in the buffers
        bluetooth.flush(uart::Queue::Both).unwrap();
    }
}

fn buzz(
    pin: gpio::Gpio,
    hz: u32,
    duration: time::Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    // start with pin in low to make sure
    pin.set_low();

    // find the number of times signal needs to be changed.  2x because both up and down need to be
    // set
    let repeats = 2 * hz as f32 * duration.as_secs() as f32;

    // find the puases needed to crete the duration without a timer
    let pause = duration / repeats;

    // set the pin to high then low with previous paramaters
    for _ in 0..repeats as u32 {
        pin.set_high();
        thread::sleep(pause);
        pin.set_low();
        thread::sleep(pause);
    }
    // Return result in case an unwrap or something similar needed later
    Ok(())
}
