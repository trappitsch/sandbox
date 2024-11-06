//! Send messages from one LPSPI peripheral to another.
//!
//! Connect lpspi3 and lpspi4 like this:
//!
//! pin 11 (lpspi4 SDO) => pin 1 (lpspi3 SDI)
//! pin 12 (lpspi4 SDI) => pin 26 (lpspi3 SDO)
//! pin 13 (lpspi4 SCK) => pin 27 (lpspi3 SCK)
//! pin 10 (lpspi4 CD)  => pin 0 (lpspi3 CS)
//!
//! Despite targeting the Teensy 4.0, this starter code
//! should also work on the Teensy 4.1 and Teensy MicroMod.
//! You should eventually target your board! See inline notes.
//!
//! This template uses [RTIC v2](https://rtic.rs/2/book/en/)
//! for structuring the application.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::{board, hal};

    use embedded_hal::{
        digital::v2::OutputPin,
        blocking::spi::{Transfer, Write}
    };
            
    use teensy4_bsp as bsp;

    use imxrt_log as logging;

    use rtic_monotonics::imxrt::prelude::*;
    imxrt_gpt1_monotonic!(Mono, board::PERCLK_FREQUENCY);

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {}

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        cs: hal::gpio::Output<bsp::pins::t40::P10>,
        spi: hal::lpspi::Lpspi<(), 4>,
        poller: logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Specify 't40', 't41', or 'tmm' (for MicroMod) depending on
        // which board you're using.
        let board::Resources {
            mut gpt1,
            mut gpio2,
            mut pins,
            usb,
            lpspi4,
            ..
        } = board::t40(cx.device);

        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        // Configure LPSPI4 clock, serial data out, and serial data in pins.
        {
            use bsp::hal::iomuxc; // <-- Pad configuration APIs are here.
            iomuxc::lpspi::prepare(&mut pins.p13); // <-- LPSPI4 SCK
            iomuxc::lpspi::prepare(&mut pins.p11); // <-- LPSPI4 SDO
            iomuxc::lpspi::prepare(&mut pins.p12); // <-- LPSPI4 SDI

            // Drop / forget pins so that they're no longer accessible.
            // We don't want to accidentally change them later.
            //
            // (There's no impl Drop on iomuxc pads, so this is basically
            // a forget.)
            //drop(pins.p13);
            //drop(pins.p12);
            //drop(pins.p11);
        }

        // Configure LPSPI4 chip select pin.
        let mut cs = gpio2.output(pins.p10);
        cs.set_high().unwrap();

        // Configure LPSPI4 peripheral.
        let mut spi = hal::lpspi::Lpspi::without_pins(lpspi4);
        spi.disabled(|spi| {
            spi.set_clock_hz(bsp::board::LPSPI_FREQUENCY, 4_000_000);
            spi.set_sample_point(hal::lpspi::SamplePoint::Edge);
        });

        //initialize monotonic
        gpt1.set_clock_source(hal::gpt::ClockSource::PeripheralClock);
        Mono::start(gpt1.release());

        spi_writer::spawn().unwrap();

        (Shared {}, Local { cs, spi, poller })
    }

    #[task(local = [cs, spi])]
    async fn spi_writer(cx: spi_writer::Context) {
        let cmd: [u8; 1] = [0b11001000];
        let mut answ: [u8; 1] = [0xFF];
        loop {
            log::info!("Querying status: cmd={:#010b}", cmd[0]);

            cx.local.cs.set_low().unwrap_or_else(|error| {
                log::error!("ERR_SPI, couldn't set chip select pin: {error:?}");
            });

            cx.local.spi.write(&cmd).unwrap_or_else(|error| {
                log::error!("ERR_SPI, couldn't write command: {error:?}");
            });

            let _ = cx.local.spi.transfer(&mut answ);

            cx.local.cs.set_high().unwrap_or_else(|error| {
                log::error!("ERR_SPI, couldn't release chip select pin: {error:?}");
            });

            log::info!("Status: answ={:#010b}", answ[0]);

            Mono::delay(1000.millis()).await;
        }
    }

    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        cx.local.poller.poll();
    }
}
