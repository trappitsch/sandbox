//! The starter code slowly blinks the LED and sets up
//! USB logging. It periodically logs messages over USB.
//!
//! Despite targeting the Teensy 4.0, this starter code
//! should also work on the Teensy 4.1 and Teensy MicroMod.
//! You should eventually target your board! See inline notes.
//!
//! This template uses [RTIC v2](https://rtic.rs/2/book/en/)
//! for structuring the application.
//!
//! Reto's Note:
//! The starter code is the template following the cargo-generate
//! instructions on the teensy4-rs repository. However, I replaced
//! the clock source code for the monotonic from the
//! rtic example repository. This clock seems to run a lot better in
//! sync than the other one.
//! Furthermore, I configured some more pins, see the common pinout
//! table for which pin is associated with which gpio port.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use core::fmt::Write;

    use bsp::{
        board,
        hal::{gpio, gpt, lpi2c},
    };
    use teensy4_bsp::{self as bsp, hal::lpi2c::Lpi2c};

    use imxrt_log as logging;

    // If you're using a Teensy 4.1 or MicroMod, you should eventually
    // change 't40' to 't41' or micromod, respectively.
    use board::t40 as my_board;

    use rtic_monotonics::imxrt::prelude::*;
    imxrt_gpt1_monotonic!(Mono, board::PERCLK_FREQUENCY);

    use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {}

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        /// The LED on pin 13.
        led: board::Led,
        lout: gpio::Output<bsp::pins::t40::P14>,
        display: Ssd1306<
            I2CInterface<Lpi2c<lpi2c::Pins<bsp::pins::t40::P19, bsp::pins::t40::P18>, 1>>,
            DisplaySize128x32,
            TerminalMode,
        >,
        count: u32,
        poller: logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio1,
            mut gpio2,
            mut gpt1,
            lpi2c1,
            pins,
            usb,
            ..
        } = my_board(cx.device);

        let lpi2c: board::Lpi2c1 =
            board::lpi2c(lpi2c1, pins.p19, pins.p18, board::Lpi2cClockSpeed::KHz400);

        let interface = I2CDisplayInterface::new(lpi2c);

        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_terminal_mode();
        display.init().unwrap();
        display.clear().unwrap();

        let count = 0;

        let led = board::led(&mut gpio2, pins.p13);
        let lout = gpio1.output(pins.p14);
        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        //initialize monotonic
        gpt1.set_clock_source(gpt::ClockSource::PeripheralClock);
        Mono::start(gpt1.release());

        blink::spawn().unwrap();
        blink2::spawn().unwrap();
        display_counter::spawn().unwrap();
        (
            Shared {},
            Local {
                led,
                lout,
                display,
                count,
                poller,
            },
        )
    }

    #[task(local = [display, count])]
    async fn display_counter(cx: display_counter::Context) {
        loop {
            cx.local.display.clear().unwrap();
            *cx.local.count = cx.local.count.wrapping_add(1);
            writeln!(cx.local.display, "Count: {}", cx.local.count).unwrap();
            writeln!(cx.local.display, "Line 2").unwrap();
            writeln!(cx.local.display, "Line 3").unwrap();
            writeln!(cx.local.display, "Line 4").unwrap();
            Mono::delay(1000.millis()).await;
        }
    }

    #[task(local = [lout])]
    async fn blink2(cx: blink2::Context) {
        loop {
            cx.local.lout.toggle();
            Mono::delay(1000.millis()).await;
            log::info!("Toggling the Pin14");
        }
    }

    #[task(local = [led], priority = 5)]
    async fn blink(cx: blink::Context) {
        let mut state = false;
        loop {
            cx.local.led.toggle();
            Mono::delay(500.millis()).await;
            state = !state;
            log::info!("Toggling the internal LED state {state}");
        }
    }

    #[task(binds = USB_OTG1, local = [poller], priority = 3)]
    fn log_over_usb(cx: log_over_usb::Context) {
        cx.local.poller.poll();
    }
}
