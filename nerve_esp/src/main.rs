#![no_std]
#![no_main]

mod blinking_ws_2812_led;

use core::fmt::Write;

use blinking_ws_2812_led::BlinkingWs2812Led;
use panic_halt as _;
use smart_leds::colors::{GREEN, PURPLE, RED};
use waveshare_rp2040_zero::{
    entry,
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        fugit::HertzU32,
        gpio::{bank0::Gpio2, FunctionSio, Pin, PullDown, SioOutput},
        pac,
        pio::PIOExt,
        timer::Timer,
        uart::{UartConfig, UartPeripheral},
        watchdog::Watchdog,
        Sio,
    },
    Gp0Uart0Tx, Gp1Uart0Rx, Pins, XOSC_CRYSTAL_FREQ,
};
use ws2812_pio::Ws2812;

use embedded_hal::digital::OutputPin;
use embedded_hal::digital::PinState::Low;

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

    // Confugure board led for debugging
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let board_led = Ws2812::new(
        // The onboard NeoPixel is attached to GPIO pin #16
        pins.neopixel.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut timer = timer;
    let mut blinking_board_led = BlinkingWs2812Led::new(board_led, &mut timer);

    // configure uart0 and transmit/receive boolean pin
    let _uart_tx: Gp0Uart0Tx = pins.gp0.into_function().into_pull_type();
    let _uart_rx: Gp1Uart0Rx = pins.gp1.into_function().into_pull_type();
    let mut uart = UartPeripheral::new(pac.UART0, (_uart_tx, _uart_rx), &mut pac.RESETS)
        .enable(
            UartConfig::new(
                HertzU32::Hz(9600),
                waveshare_rp2040_zero::hal::uart::DataBits::Eight,
                None,
                waveshare_rp2040_zero::hal::uart::StopBits::One,
            ),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    let mut direction_pin: Pin<Gpio2, FunctionSio<SioOutput>, PullDown> = pins
        .gp2
        .into_function::<FunctionSio<SioOutput>>()
        .into_push_pull_output_in_state(Low);

    // Main loop
    loop {
        match direction_pin.set_high() {
            Ok(()) => {
                blinking_board_led.set_static(GREEN, 32).unwrap();
                match uart.write_str("hello world!") {
                    Ok(()) => (),
                    Err(_) => blinking_board_led.blink_times(200, 20, PURPLE, 32).unwrap(),
                }
            }
            Err(_) => blinking_board_led.blink_times(200, 20, PURPLE, 32).unwrap(),
        }
        match direction_pin.set_low() {
            Ok(()) => {
                blinking_board_led.blink_times(2000, 1, RED, 32).unwrap();
            }
            Err(_) => blinking_board_led.blink_times(200, 20, PURPLE, 32).unwrap(),
        }
    }
}
