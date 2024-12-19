#![no_std]
#![no_main]

use core::fmt::Write;

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    prelude::*,
    uart::Uart,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    //let mut usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);
    //let (tx_pin, rx_pin) = (peripherals.GPIO21, peripherals.GPIO20);
    //let mut uart1 = Uart::new(peripherals.UART1, tx_pin, rx_pin).unwrap();

    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(peripherals.GPIO0, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let delay = Delay::new();

    let min = 1630;
    let max = 4095;
    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();

        let pin_value: f64 = (pin_value - min) as f64 / (max - min) as f64;

        delay.delay_millis(1000);

        //uart1.write_str("Hallo").unwrap();
        //uart1.flush_tx().unwrap();
        println!("value {}", pin_value);

        //usb_serial.write_bytes(b"Hello, world!").expect("write error!");


    }
}
