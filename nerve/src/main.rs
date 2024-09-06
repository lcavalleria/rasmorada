#![no_std]
#![no_main]

use core::iter::once;
use panic_halt as _;
use smart_leds::{
    brightness,
    colors::{BLUE, GREEN, RED},
};
use smart_leds_trait::SmartLedsWrite;
use waveshare_rp2040_zero::{
    entry,
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use ws2812_pio::Ws2812;

use embedded_hal::delay::DelayNs;

#[entry]
fn main() -> ! {
    // Setup
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Confugure led
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        // The onboard NeoPixel is attached to GPIO pin #16
        pins.neopixel.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut timer = timer;

    loop {
        ws.write(brightness(once(RED), 32)).unwrap();
        timer.delay_ms(500);
        ws.write(brightness(once(GREEN), 32)).unwrap();
        timer.delay_ms(500);
        ws.write(brightness(once(BLUE), 32)).unwrap();
        timer.delay_ms(500);
    }
}
