#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f3xx_hal::{prelude::*, stm32};
// use stm32f3::stm32f303;
// use stm32f303::interrupt;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

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
    let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb);
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
    usart1_tx.write(1u8).unwrap();
    loop {
        let mut recieved = false;
        if let Ok(byte) = usart1_rx.read() {
            if byte == 1u8 {
                recieved = true
            }
        }
        if recieved {}
    }
}
