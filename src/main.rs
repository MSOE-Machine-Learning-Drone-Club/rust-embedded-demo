use std::error::Error;

use log::logger;


fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello :))))");

    /*let peripherals = Peripherals::take()?;
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
        FreeRtos::delay_ms(20);
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
        //log::info!("{}", res);
        if res == 1.0 {
            led.set_high()?;
        } else {
            led.set_low()?;
        }
    }
    let camera_config = esp_idf_sys::camera::camera_config_t {
        pin_pwdn: -1,
        pin_reset: -1,
        pin_xclk: 15,
        sccb_i2c_port: -1,
        __bindgen_anon_1:
            esp_idf_sys::camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: 4
            },
        __bindgen_anon_2:
            esp_idf_sys::camera::camera_config_t__bindgen_ty_2 {
                pin_sscb_scl: 5,
            },
        pin_d7: 16,
        pin_d6: 17,
        pin_d5: 18,
        pin_d4: 12,
        pin_d3: 10,
        pin_d2: 8,
        pin_d1: 9,
        pin_d0: 11,
        pin_vsync: 6,
        pin_href: 7,
        pin_pclk: 13,
        xclk_freq_hz: 20000000,
        ledc_timer: esp_idf_sys::ledc_timer_t_LEDC_TIMER_0,
        ledc_channel: esp_idf_sys::ledc_channel_t_LEDC_CHANNEL_0,
        pixel_format: esp_idf_sys::cam::pixformat_t_PIXFORMAT_JPEG,
        frame_size: esp_idf_sys::cam::framesize_t_FRAMESIZE_UXGA,
        jpeg_quality: 12,
        fb_count: 1,
        fb_location:
            esp_idf_sys::cam::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
        grab_mode:
            esp_idf_sys::cam::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,
    };
    unsafe {
        if esp_idf_sys::camera::esp_camera_init(&camera_config) != 0 {
            println!("camera init failed!");
            return Ok(());
        } else {
            println!("camera ready!");
        }
        let fb = esp_idf_sys::camera::esp_camera_fb_get();
        println!("Picture taken! Its size was: {} bytes", unsafe {
            (*fb).len
        });
        let data = std::slice::from_raw_parts((*fb).buf, (*fb).len);
        println!("{data:?}");
    }
    */
    Ok(())
}
