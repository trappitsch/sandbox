//! This code is based on the teensy4-rs template for the defmt logger
//!
//! It demonstrates a simple loopback device, i.e.,
//! it reads a given command into a buffer and as soon as a newline
//! is found, sends the buffer back with a newline character appended.
//!
//! Note: When testing with picocom, make sure to use the following mappigns:
//! --omap crlf
//! --imap lfcrlf
//! These mappings are necessary since we assume a simple b'\n' as the terminator.
//!
//!
//! The following parts would need to be implemented to make this production ready:
//! - Error handling...
//! - Make sure the buffer does not overflow
//! - Implement a proper command parser

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::{
        board,
        hal::usbd::{
            gpt::{Instance::Gpt0, Mode},
            BusAdapter, EndpointMemory, EndpointState, Speed,
        },
    };
    use teensy4_bsp as bsp;

    use usb_device::{
        bus::UsbBusAllocator,
        device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    };
    use usbd_serial::SerialPort;

    use rtic_monotonics::systick::*;

    /// We're intentionally using a full-speed device instead of a high-speed
    /// device. The full-speed device has better support in the usb-device
    /// ecosystem (in terms of packages and host support), and we don't need a
    /// high-speed device for this efficient logging.
    const SPEED: Speed = Speed::LowFull;
    /// Looking for USB devices on your host? Search for this VID and PID.
    const VID_PID: UsbVidPid = UsbVidPid(0x5824, 0x27dd);
    /// You could also look for this product ID.
    const PRODUCT: &str = "teensy4-bsp-example";

    const TERMINATOR: u8 = b'\n';

    #[local]
    struct Local {
        usb_class: SerialPort<'static, BusAdapter>,
        usb_device: UsbDevice<'static, BusAdapter>,
        led: board::Led,
    }

    #[shared]
    struct Shared {
        read_buffer: [u8; 256],
        read_index: usize,
    }

    #[init(local = [
        ep_memory: EndpointMemory<1024> = EndpointMemory::new(),
        ep_state: EndpointState = EndpointState::max_endpoints(),
        usb_bus: Option<UsbBusAllocator<BusAdapter>> = None,
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            pit: (mut timer, _, _, _),
            usb,
            pins,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);

        let bus_adapter = BusAdapter::with_speed(usb, cx.local.ep_memory, cx.local.ep_state, SPEED);
        bus_adapter.set_interrupts(true);
        bus_adapter.gpt_mut(Gpt0, |gpt| {
            gpt.stop();
            gpt.clear_elapsed();
            gpt.set_interrupt_enabled(true);
            gpt.set_mode(Mode::Repeat);
            gpt.set_load(10_000); // microseconds.
            gpt.reset();
            gpt.run();
        });

        timer.set_load_timer_value(board::PERCLK_FREQUENCY / 1000);
        timer.set_interrupt_enable(true);
        timer.enable();

        let usb_bus = cx.local.usb_bus.insert(UsbBusAllocator::new(bus_adapter));
        let usb_class = SerialPort::new(usb_bus);
        let usb_device = UsbDeviceBuilder::new(usb_bus, VID_PID)
            .product(PRODUCT)
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();

        // initialize read buffer and index
        let read_buffer = [0; 256];
        let read_index = 0;

        // Set up a system timer for our software task.
        {
            Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }

        (
            Shared {
                read_buffer,
                read_index,
            },
            Local {
                usb_class,
                usb_device,
                led,
            },
        )
    }

    #[task(binds = USB_OTG1, local = [usb_class, usb_device, led, configured: bool = false], shared=[read_buffer, read_index])]
    fn usb_interrupt(mut cx: usb_interrupt::Context) {
        let usb_interrupt::LocalResources {
            usb_class,
            usb_device,
            led,
            configured,
            ..
        } = cx.local;

        if usb_device.poll(&mut [usb_class]) {
            if usb_device.state() == UsbDeviceState::Configured {
                if !*configured {
                    usb_device.bus().configure();
                }
                *configured = true;

                let mut buffer = [0; 256];
                match usb_class.read(&mut buffer) {
                    Ok(count) => {
                        led.clear();
                        cx.shared.read_index.lock(|ind| {
                            cx.shared.read_buffer.lock(|buf| {
                                for chr in buffer.iter().take(count) {
                                    if *chr == TERMINATOR {
                                        led.toggle();
                                        let _ = usb_class.write(&buf[..*ind]);
                                        let _ = usb_class.write(&[TERMINATOR]);
                                        *ind = 0; // no need to clear the buffer...
                                    } else {
                                        buf[*ind] = *chr;
                                        *ind += 1;
                                    };
                                };
                            });
                        });
                    }
                    Err(_) => {
                        led.toggle();
                    }
                }
            } else {
                *configured = false;
            }
        }
    }
}
