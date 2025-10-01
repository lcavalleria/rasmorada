mod uart;

use core::str;
use uart::BrainUart;

#[tokio::main]
async fn main() {
    println!("Started Brain");
    // init
    let mut uart: BrainUart = BrainUart::uart_init();
    let mut buffer = [0u8; 32];

    let read_uart = loop {
        match uart.read() {
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
    };
}
