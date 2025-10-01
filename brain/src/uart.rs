use rppal::gpio::{Gpio, OutputPin};
use rppal::uart::{Parity, Uart};

pub struct BrainUart {
    uart: Uart,
    send_mode: bool,
    buffer: [u8; 32],
}

impl BrainUart {
    pub fn uart_init() -> Self {
        let uart = Uart::with_path("/dev/ttyAMA0", 9600, Parity::None, 8, 1)
            .expect("Failed to initialize UART");

        let gpio = Gpio::new().expect("Failed to initialize GPIO");
        let direction_pin: OutputPin = gpio.get(17).expect("Failed to get Gpio0").into_output_low();
        let buffer = [0u8; 32];

        return Self {
            uart: uart,
            send_mode: false,
            buffer: buffer,
        };
    }

    fn set_send_mode(&mut self, send_mode: bool) {
        self.send_mode = send_mode;
    }

    pub fn read(&mut self) -> Result<usize, rppal::uart::Error> {
        return self.uart.read(&mut self.buffer);
    }
    pub fn send(&mut self, buffer: &[u8]) -> Result<usize, rppal::uart::Error> {
        self.set_send_mode(false);
        let res: Result<usize, rppal::uart::Error> = self.uart.write(buffer);
        self.set_send_mode(true);
        res
    }
}
