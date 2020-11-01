#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_itm as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    prelude::*,
    serial::{config, Serial},
    stm32, time,
};

#[entry]
fn main() -> ! {
    // pulling peripherals
    let peripherals = stm32::Peripherals::take().unwrap();
    // using rcc
    let mut rcc = peripherals.RCC.constrain();

    // clock for usart1 timiing
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    // setup gpioa for the tx and rx pins for the HC-05 bluetooth board
    let mut gpioa = peripherals.GPIOA.split();
    // setup gpiob for the button
    let mut gpiob = peripherals.GPIOB.split();

    // create pull down input button pin on pb2
    let button = gpiob.pb10.into_pull_down_input();

    // create tx and rx pins with alternative funcction 7
    // USART1 is found as AF07 within datasheet
    let usart1_txd = gpioa.pa9.into_alternate_af7();
    let usart1_rxd = gpioa.pa10.into_alternate_af7();

    // setup bluetooth config
    let mut bluetooth_config = config::Config {
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
    let (mut usart1_tx, mut usart1_rx) = usart1.split();

    // used later to display whether or not a signal was received
    let mut recieved = false;
    loop {
        // Below is for debugging
        //if button.is_low().unwrap() {
        //    usart1_tx.bwrite_all(&b"LOW"[..]).unwrap();
        //    usart1_tx.bflush().unwrap();
        //} else {
        //    if button.is_high().unwrap() {
        //        usart1_tx.bwrite_all(&b"HIGH"[..]).unwrap();
        //        usart1_tx.bflush().unwrap();
        //    } else {
        //        usart1_tx.bwrite_all(&b"NONE"[..]).unwrap();
        //        usart1_tx.bflush().unwrap();
        //    }
        //}

        // While button is not pressed, wait
        while button.is_low().unwrap() {}

        // When button is pressed send a BUZZ signal
        usart1_tx.bwrite_all(&b"BUZZ"[..]).unwrap();
        usart1_tx.bflush().unwrap();

        // for future relay of received
        if let Ok(_) = usart1_rx.read() {
            recieved = true;
        }
        if recieved {
            // light on will go here in the future
            usart1_tx.bwrite_all(&b"RECEIVED"[..]).unwrap();
            usart1_tx.bflush().unwrap();
            recieved = false
        }
    }
}
