use core::iter::once;

use embedded_hal::delay::DelayNs;
use smart_leds::{brightness, RGB8};
use smart_leds_trait::SmartLedsWrite;
use waveshare_rp2040_zero::hal::gpio::{Pin, PullDown};
use waveshare_rp2040_zero::hal::pac;
use waveshare_rp2040_zero::hal::timer::Timer;
use ws2812_pio::Ws2812;

pub struct BlinkingWs2812Led<'a> {
    led: Ws2812<
        pac::PIO0,
        waveshare_rp2040_zero::hal::pio::SM0,
        waveshare_rp2040_zero::hal::timer::CountDown<'a>,
        Pin<
            waveshare_rp2040_zero::hal::gpio::bank0::Gpio16,
            waveshare_rp2040_zero::hal::gpio::FunctionPio0,
            PullDown,
        >,
    >,
    timer: &'a mut Timer,
}

impl<'a> BlinkingWs2812Led<'a> {
    pub fn new(
        led: Ws2812<
            pac::PIO0,
            waveshare_rp2040_zero::hal::pio::SM0,
            waveshare_rp2040_zero::hal::timer::CountDown<'a>,
            Pin<
                waveshare_rp2040_zero::hal::gpio::bank0::Gpio16,
                waveshare_rp2040_zero::hal::gpio::FunctionPio0,
                PullDown,
            >,
        >,
        timer: &'a mut Timer,
    ) -> Self {
        Self { led, timer }
    }

    pub fn blink_times(
        &mut self,
        blink_ms: u32,
        times: u8,
        color: RGB8,
        led_brightness: u8,
    ) -> Result<(), ()> {
        for i in 0..times {
            self.led
                .write(brightness(once(color), led_brightness * ((i + 1) % 2)))?;
            self.timer.delay_ms(blink_ms);
        }
        Ok(())
    }

    pub fn set_static(&mut self, color: RGB8, led_brightness: u8) -> Result<(), ()> {
        self.led.write(brightness(once(color), led_brightness))?;
        self.timer.delay_ms(50); // We cannot call write again until we trigger the timer delay
        Ok(())
    }
}
