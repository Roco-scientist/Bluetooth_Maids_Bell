#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_itm as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use rtic::app;
use stm32f3xx_hal::{
    gpio::{gpiob::PB2, Input, PullDown},
    prelude::*,
    serial::{Rx, Tx},
    stm32,
};

#[rtic::app(device = stm32f3xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        bluetooth_tx: Tx<stm32::USART1>,
        bluetooth_rx: Rx<stm32::USART1>,
        button: PB2<Input<PullDown>>,
    }
    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        // pulling peripherals
        let peripherals: stm32::Peripherals = cx.device;
        // using rcc
        let mut rcc = peripherals.RCC.constrain();
        // flash for the clock
        let mut flash = peripherals.FLASH.constrain();

        // clock for usart1 timiing
        let clocks = rcc.cfgr.use_hse(8.mhz()).freeze(&mut flash.acr);

        // setup gpioa for the tx and rx pins for the HC-05 bluetooth board
        let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
        // setup gpiob for the button
        let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);

        // create pull down input button pin on pb2
        let button = gpiob
            .pb2
            .into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr);

        // make button push into an interrupt
        peripherals
            .SYSCFG
            .exticr1
            .write(|w| unsafe { w.exti2().bits(1) });

        // create tx and rx pins with alternative funcction 7
        let usart1_txd = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
        let usart1_rxd = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

        // setup usart with tx and rx pins, along with bus and clocks
        let usart1 = stm32f3xx_hal::serial::Serial::usart1(
            peripherals.USART1,
            (usart1_txd, usart1_rxd),
            9600.bps(),
            clocks,
            &mut rcc.apb2,
        );

        // split out the tx and rx communication of the bluetooth
        let (bluetooth_tx, bluetooth_rx) = usart1.split();
        init::LateResources {
            bluetooth_tx,
            bluetooth_rx,
            button,
        }
    }
    #[task(binds = EXTI1, resources = [button, bluetooth_tx])]
    fn exti_3_10_interrupt(ctx: exti_3_10_interrupt::Context) {
        // When button is pressed send a BUZZ signal
        ctx.resources.bluetooth_tx.bwrite_all(&b"BUZZ"[..]).unwrap();
        ctx.resources.bluetooth_tx.bflush().unwrap();
    }
};
