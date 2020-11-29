#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_itm as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m::peripheral::NVIC;
use rtic::app;
use stm32f4xx_hal::{
    block,
    gpio::{gpioa::PA4, Input, PullDown},
    prelude::*,
    serial::{config, Rx, Serial, Tx},
    stm32, time,
};

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        bluetooth_tx: Tx<stm32::USART1>,
        bluetooth_rx: Rx<stm32::USART1>,
        button: PA4<Input<PullDown>>,
        exti: stm32::EXTI,
    }
    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        // pulling peripherals
        let peripherals: stm32::Peripherals = cx.device;
        // enable syscfg for interrupt below
        peripherals.RCC.apb2enr.write(|w| w.syscfgen().enabled());
        // using rcc
        let rcc = peripherals.RCC.constrain();

        // clock for usart1 timing, using HSE at 25mhz
        let clocks = rcc.cfgr.use_hse(25.mhz()).freeze();

        // setup gpioa for the tx and rx pins for the HC-05 bluetooth board
        let gpioa = peripherals.GPIOA.split();

        // create pull down input button pin on pa4
        // https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/stopwatch-with-ssd1306-and-interrupts.rs
        let button = gpioa.pa4.into_pull_down_input();
        // button.make_interrupt_source(&mut peripherals.SYSCFG);
        // button.enable_interrupt(&mut peripherals.EXTI);
        // button.trigger_on_edge(&mut peripherals.EXTI, Edge::RISING);

        // set pa4 as an external rising trigger interrupt
        // sets the rtsr at an offset of 8
        // make button push into an interrupt
        let syscfg = &peripherals.SYSCFG;
        syscfg.exticr2.write(|w| unsafe { w.exti4().bits(0b0000) }); // per the manual 0000 indicates pa4 on exti4 for exticr2

        // from: https://flowdsp.io/blog/stm32f3-01-interrupts/
        let exti = peripherals.EXTI;
        exti.imr.modify(|_, w| w.mr4().set_bit()); // unmask interrupt
        exti.rtsr.modify(|_, w| w.tr4().set_bit()); // trigger on rising-edge

        // connect the interrupt to NVIC
        NVIC::unpend(stm32::interrupt::EXTI2);
        unsafe { NVIC::unmask(stm32::interrupt::EXTI2) };

        // create tx and rx pins with alternative funcction 7
        // USART1 is found as AF07 within datasheet
        let usart1_txd = gpioa.pa9.into_alternate_af7();
        let usart1_rxd = gpioa.pa10.into_alternate_af7();

        // setup bluetooth config
        let bluetooth_config = config::Config {
            baudrate: time::Bps(115200),
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
        let (bluetooth_tx, bluetooth_rx) = usart1.split();
        init::LateResources {
            bluetooth_tx,
            bluetooth_rx,
            button,
            exti,
        }
    }

    #[task(binds = EXTI2, resources = [button, bluetooth_tx, exti])]
    fn exti_2_4_interrupt(ctx: exti_2_4_interrupt::Context) {
        // mask interrupt so it doesn't occur while this is happening
        NVIC::mask(stm32::interrupt::EXTI2);
        // When button is pressed send a BUZZ signal
        for byte in b"BUZZ" {
            block!(ctx.resources.bluetooth_tx.write(*byte)).unwrap();
        }
        // flush what is lef in the registrar
        block!(ctx.resources.bluetooth_tx.flush()).unwrap();
        // ctx.resources.button.clear_interrupt_pending_bit();

        // clear interrupt on pending registrar
        ctx.resources.exti.pr.modify(|_, w| w.pr4().clear());
        // unmask the interrupt
        unsafe { NVIC::unmask(stm32::interrupt::EXTI2) };
    }
};
