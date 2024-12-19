#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    analog::adc::{AdcConfig, Attenuation, Adc},
    usb_serial_jtag::UsbSerialJtag,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);

    // Set GPIO0 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let analog_pin = io.pins.gpio0;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(analog_pin,
                  Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let delay = Delay::new(&clocks);

    let min = 1630;
    let max = 4095;
    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();

        let pin_value: f64 = (pin_value - min) as f64 / (max - min) as f64;

        delay.delay_millis(1000);

        println!("value {}", pin_value);

        //usb_serial.write_bytes(b"Hello, world!").expect("write error!");
        //tx.write_bytes(&pin_value.to_le_bytes()).expect("write error!");
        //println!("middle");
        //let byte = rx.read_byte().expect("read error!");

        //println!("byte {}", byte);
    }
}
