//! Have a button connected to pin 4 and on a hardware interrupt.
//! Pressing the button will increase a counter and blink the LED for 250 ms.
//! Every 3 second, the counter is logged over USB using a periodic interrupt timer.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use board::t40 as my_board;
    use bsp::{
        board,
        hal::{gpio, gpt, iomuxc, snvs},
        ral,
    };
    use imxrt_log as logging;
    use teensy4_bsp as bsp;

    const GPT_CLOCK_SOURCE: gpt::ClockSource = gpt::ClockSource::PeripheralClock;
    const GPT_DIVIDER: u32 = 8;

    const PIT_DELAY_MS: u32 = board::PERCLK_FREQUENCY / 1_000 * 1_000;

    use rtic_monotonics::imxrt::prelude::*;
    imxrt_gpt1_monotonic!(Mono, board::PERCLK_FREQUENCY);

    /// There are no resources shared across tasks.
    #[shared]
    struct Shared {
        counter: u32,
        gpt: gpt::Gpt2,
        srtc: snvs::srtc::Srtc,
    }

    /// These resources are local to individual tasks.
    #[local]
    struct Local {
        led: board::Led,
        button: gpio::Input<bsp::pins::t40::P4>,
        poller: logging::Poller,
        debounce_cycles: u32,
        pit: bsp::hal::pit::Pit<2>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio2,
            mut gpio4,
            mut gpt1,
            mut gpt2,
            mut pins,
            pit: (_, _, mut pit, _),
            usb,
            ..
        } = my_board(cx.device);

        let snvs::Snvs {
            low_power: snvs::LowPower { mut core, srtc, .. },
            ..
        } = snvs::new(unsafe { ral::snvs::SNVS::instance() });
        let srtc = srtc.enable_and_set(&mut core, 1600000000, 0);

        iomuxc::configure(
            &mut pins.p4,
            iomuxc::Config::zero()
                .set_hysteresis(iomuxc::Hysteresis::Disabled)
                .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k)),
        );
        let button = gpio4.input(pins.p4);
        gpio4.set_interrupt(&button, Some(gpio::Trigger::FallingEdge));

        let led = board::led(&mut gpio2, pins.p13);

        let counter = 0;

        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(PIT_DELAY_MS);
        pit.enable();

        gpt1.set_clock_source(gpt::ClockSource::PeripheralClock);
        Mono::start(gpt1.release());

        gpt2.set_clock_source(GPT_CLOCK_SOURCE);
        gpt2.set_divider(GPT_DIVIDER);
        gpt2.enable();
        let gpt = gpt2;
        let debounce_cycles = 0;

        (
            Shared { counter, gpt, srtc },
            Local {
                debounce_cycles,
                led,
                button,
                poller,
                pit,
            },
        )
    }

    #[task(shared = [counter], local = [led])]
    async fn blink_led(cx: blink_led::Context) {
        cx.local.led.set();
        Mono::delay(250.millis()).await;
        cx.local.led.clear();
    }

    #[task(binds = GPIO4_COMBINED_0_15, shared = [counter, gpt], local = [button, debounce_cycles])]
    fn button_pressed(mut cx: button_pressed::Context) {
        // some complicated debounce routine...
        let mut debounce_active = false;
        cx.shared.gpt.lock(|gpt| {
            let current_time = gpt.count();
            // setup debounce time for the first time
            if *cx.local.debounce_cycles == 0 {
                // first press
                *cx.local.debounce_cycles = current_time + 100_000;
                log::info!("First press");
            } else if current_time > *cx.local.debounce_cycles {
                // second press debounced
                *cx.local.debounce_cycles = 0;
                log::info!("Second press - debounced");
            } else {
                // not debounced yet
                debounce_active = true;
                log::info!("Debouncing");
            }
            log::info!(
                "Debounce time: {}, Current Time: {}",
                *cx.local.debounce_cycles,
                current_time
            );
        });
        cx.local.button.clear_triggered();
        log::info!("debounce_active: {}", debounce_active);

        if !debounce_active {
            cx.shared.counter.lock(|cnt| {
                *cnt += 1;
            });
            blink_led::spawn().ok();
        }
    }

    #[task(binds = PIT, local = [pit], shared = [counter, srtc], priority=2)]
    fn log_counter(mut cx: log_counter::Context) {
        let pit = cx.local.pit;

        let mut time_now: (u32, u32) = (0, 0);
        cx.shared.srtc.lock(|srtc| {
            time_now = srtc.get_with_micros();
        });
        cx.shared.counter.lock(|cnt| {
            log::info!("Counter: {}, Time: {:?}", *cnt, time_now);
        });

        while pit.is_elapsed() {
            pit.clear_elapsed();
        }
    }

    #[task(binds = USB_OTG1, local = [poller], priority=10)]
    fn log_over_usb(cx: log_over_usb::Context) {
        cx.local.poller.poll();
    }
}
