use rppal::uart;
use std::time;

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

    loop{
        // create empty data array to put read data into
        let mut data = [0u8; 32usize];

        // Wait for signal to come from sender and put read into data array
        bluetooth.read(&mut data).unwrap();

        // print for debugging
        println!("{:?}", data);

        // return a signal of receipt and confirmation
        bluetooth.write(&b"RECEIVED"[..]).unwrap();

        // flush anything else remaining in the buffers
        bluetooth.flush(uart::Queue::Both).unwrap();
    }
}
