use core::str;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};

fn main() {
    println!("Started Brain");
    let mut uart = Uart::with_path("/dev/ttyAMA0", 9600, Parity::None, 8, 1)
        .expect("Failed to initialize UART");

    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let _direction_pin = gpio.get(0).expect("Failed to get Gpio0").into_output_low();

    let mut buffer = [0u8; 8];
    loop {
        match uart.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    println!("Read {} bytes", bytes_read);
                    println!("Buffer: {:02x?}", &buffer);

                    if let Ok(msg) = str::from_utf8(&buffer[..bytes_read]) {
                        if msg.starts_with('h') {
                            println!("YES")
                        }
                        println!("Read message: {}", msg);
                    } else {
                        println!("Received non-UTF8 data.");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read uart into buffer{:?}:", e);
                break;
            }
        }
    }
}
