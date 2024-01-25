use std::error::Error;

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_triton::network::network::Network;
use esp_idf_hal::gpio::*;

fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio12)?;

    let mut button1 = PinDriver::input(peripherals.pins.gpio4)?;
    let mut button2 = PinDriver::input(peripherals.pins.gpio5)?;

    button1.set_pull(Pull::Down)?;
    button2.set_pull(Pull::Down)?;

    let model_str = "D|3|2|10.845654 11.002682 -13.501029 -14.699452 -53.440483 -53.715294|-6.101849 49.06853 61.28852#D|1|3|30.350481 -78.40228 70.861206|-19.532055#D|1|1|15.161753|-3.7315714".to_string();

    let mut xor_net = Network::deserialize_triton_fmt_string(model_str);

    let mut input1: f32;
    let mut input2: f32;

    loop {
        FreeRtos::delay_ms(2000);
        if button1.is_high() {
            input1 = 0.0;
        } else {
            input1 = 1.0;
        }

        if button2.is_high() {
            input2 = 0.0;
        } else {
            input2 = 1.0;
        }

        let res = xor_net.predict(&vec![input1, input2])[0].round();
        log::info!("{}", res);
        if res == 1.0 {
            led.set_high()?;
        } else {
            led.set_low()?;
        }
    }
}
