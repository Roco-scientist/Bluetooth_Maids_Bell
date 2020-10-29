#![no_main]
#![no_std]

#[macro_use(block)]
extern crate nb;

#[cfg(debug_assertions)]
use panic_itm as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m_rt::entry;
use stm32f3xx_hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    // pulling peripherals
    let peripherals = stm32::Peripherals::take().unwrap();
    // using rcc
    let mut rcc = peripherals.RCC.constrain();
    let mut flash = peripherals.FLASH.constrain();

    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze(&mut flash.acr);

    // usart1 is on apb2enr bus.  Writing and enabling usart1 enable bit
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let button = gpiob
        .pb2
        .into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let usart1_txd = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let usart1_rxd = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let usart1 = stm32f3xx_hal::serial::Serial::usart1(
        peripherals.USART1,
        (usart1_txd, usart1_rxd),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );
    let (mut usart1_tx, mut usart1_rx) = usart1.split();
    let mut tim6 =
        stm32f3xx_hal::timer::Timer::tim6(peripherals.TIM6, 8.mhz(), clocks, &mut rcc.apb1);
    tim6.start(8.mhz());
    block!(tim6.wait());
    let mut recieved = false;
    loop {
        if button.is_low().unwrap() {
            usart1_tx.bwrite_all(&b"LOW"[..]).unwrap();
            usart1_tx.bflush().unwrap();
        } else {
            if button.is_high().unwrap() {
                for byte in b"HIGH" {
                    usart1_tx.write(*byte).unwrap();
                }
            } else {
                for byte in b"NONE" {
                    usart1_tx.write(*byte).unwrap();
                }
            }
        }
        while button.is_low().unwrap() {}
        usart1_tx.bwrite_all(&b"BUZZ"[..]).unwrap();
        usart1_tx.bflush().unwrap();
        if let Ok(_) = usart1_rx.read() {
            recieved = true;
        }
        if recieved {
            usart1_tx.bwrite_all(&b"RECEIVED"[..]).unwrap();
            usart1_tx.bflush().unwrap();
            recieved = false
        }
    }
}
