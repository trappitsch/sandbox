//! Use a button to drag a pin from 3.3V to GND.
//! When button is pressed, print a message to USB and light LED.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use board::t40 as my_board;
    use bsp::{
        board,
        hal::{gpio, iomuxc},
    };
    use imxrt_log as logging;
    use teensy4_bsp as bsp;

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {}

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        led: board::Led,
        button: gpio::Input<bsp::pins::t40::P4>,
        poller: logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio2,
            mut gpio4,
            mut pins,
            usb,
            ..
        } = my_board(cx.device);

        iomuxc::configure(
            &mut pins.p4,
            iomuxc::Config::zero()
                .set_hysteresis(iomuxc::Hysteresis::Disabled)
                .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k)),
        );
        let button = gpio4.input(pins.p4);
        gpio4.set_interrupt(&button, Some(gpio::Trigger::EitherEdge));

        let led = board::led(&mut gpio2, pins.p13);

        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        (
            Shared {},
            Local {
                led,
                button,
                poller,
            },
        )
    }

    #[task(binds = GPIO4_COMBINED_0_15, local = [led, button])]
    fn button_pressed(cx: button_pressed::Context) {
        cx.local.button.clear_triggered();
        if !cx.local.button.is_set() {
            log::info!("Button pressed!");
            cx.local.led.set();
        } else {
            log::info!("Nope!");
            cx.local.led.clear();
        }
    }

    #[task(binds = USB_OTG1, local = [poller], priority=2)]
    fn log_over_usb(cx: log_over_usb::Context) {
        cx.local.poller.poll();
    }
}
