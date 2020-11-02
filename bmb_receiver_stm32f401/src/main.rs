#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_itm as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    block,
    delay::Delay,
    gpio,
    prelude::*,
    serial::{config, Serial},
    stm32, time,
};

#[entry]
fn main() -> ! {
    // pulling peripherals
    let peripherals = stm32::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    // using rcc
    let rcc = peripherals.RCC.constrain();

    // clock for usart1 timiing
    let clocks = rcc.cfgr.freeze();
    //
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);

    // setup gpioa for the tx and rx pins for the HC-05 bluetooth board
    let gpioa = peripherals.GPIOA.split();
    // setup gpiob for the button
    let gpiob = peripherals.GPIOB.split();

    let mut buzzer_pin = gpiob.pb9.into_push_pull_output();

    // create tx and rx pins with alternative funcction 7
    // USART1 is found as AF07 within datasheet
    let usart1_txd = gpioa.pa9.into_alternate_af7();
    let usart1_rxd = gpioa.pa10.into_alternate_af7();

    // setup bluetooth config
    let bluetooth_config = config::Config {
        baudrate: time::Bps(9600),
        wordlength: config::WordLength::DataBits8,
        parity: config::Parity::ParityNone,
        stopbits: config::StopBits::STOP1,
    };

    // setup usart with tx and rx pins, along with bus and clocks
    let usart1 = Serial::usart1(
        peripherals.USART1,
        (usart1_txd, usart1_rxd),
        bluetooth_config,
        clocks,
    )
    .unwrap();

    // split out the tx and rx communication of the bluetooth
    let (_usart1_tx, mut usart1_rx) = usart1.split();

    loop {
        // create empty data array to put read data into
        let mut data = [0u8; 32];

        // make sure at least 4 bytes are received before going forward
        for x in 0..4 {
            // Wait for signal to come from sender and put the read into data vector
            data[x] = block!(usart1_rx.read()).unwrap();
        }

        // run buzzer signal
        buzz(&mut buzzer_pin, 300, &mut delay, 500);
        delay.delay_ms(500u32);
        buzz(&mut buzzer_pin, 1000, &mut delay, 500);
        delay.delay_ms(500u32);
        buzz(&mut buzzer_pin, 200, &mut delay, 500);
    }
}

fn buzz(
    pin: &mut gpio::gpiob::PB9<gpio::Output<gpio::PushPull>>,
    hz: u32,
    delay: &mut Delay,
    duration: u32,
) -> () {
    // start with pin in low to make sure
    pin.set_low().unwrap();

    // find the number of times signal needs to be changed.  2x because both up and down need to be
    // set
    let repeats = (hz as f32 * duration as f32 / 1000f32) as u32;

    // find the puases needed to crete the duration without a timer
    let pause = duration / repeats;

    // set the pin to high then low with previous paramaters
    for _ in 0..repeats {
        pin.set_high().unwrap();
        delay.delay_ms(pause);
        pin.set_low().unwrap();
        delay.delay_ms(pause);
    }
}
