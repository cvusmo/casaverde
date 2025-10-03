#![no_std]

// use core::{ptr, slice};

// GPIO & UART Regsiter Definitions
mod registers {
    // GPIO registers
    pub const DDRD: *mut u8 = 0x2A as *mut u8;
    pub const PORTD: *mut u8 = 0x2B as *mut u8;
    pub const RELAY_PIN: u8 = 1 << 3;

    // UART Registers - 16MHz with 9600 baud
    pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;
    pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;
    pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;
    pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;
    pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;
    pub const UDR0: *mut u8 = 0xC6 as *mut u8;
}

// Hardware Abstraction Layer
mod hal {
    use super::registers;
    use core::ptr;

    pub struct Relay {
        pin: u8,
    }

    impl Relay {
        pub fn new(pin: u8) -> Self {
            Relay { pin }
        }

        pub fn init(&self) {
            unsafe {
                let ddrd = ptr::read_volatile(registers::DDRD);
                ptr::write_volatile(registers::DDRD, ddrd | self.pin);
            }
        }

        pub fn set_high(&self) {
            unsafe {
                let portd = ptr::read_volatile(registers::PORTD);
                ptr::write_volatile(registers::PORTD, portd | self.pin);
            }
        }

        pub fn set_low(&self) {
            unsafe {
                let portd = ptr::read_volatile(registers::PORTD);
                ptr::write_volatile(registers::PORTD, portd & !self.pin);
            }
        }
    }

    pub struct Uart {
        initialized: bool,
    }

    impl Uart {
        pub fn new() -> Self {
            Uart { initialized: false }
        }

        pub fn init(&mut self) {
            if !self.initialized {
                unsafe {
                    // Set baud rate to 9600
                    ptr::write_volatile(registers::UBRR0H, 0); // High byte
                    ptr::write_volatile(registers::UBRR0L, 103); // Low byte

                    // Enable Tx/Rx
                    ptr::write_volatile(registers::UCSR0B, 0b10011000);

                    // Set frame
                    ptr::write_volatile(registers::UCSR0C, 0b00000110);
                }
                self.initialized = true;
            }
        }

        pub fn read(&self, buffer: &mut [u8]) -> usize {
            if buffer.is_empty() {
                return 0;
            }
            unsafe {
                if ptr::read_volatile(registers::UCSR0A) & (1 << 7) != 0 {
                    let byte = ptr::read_volatile(registers::UDR0);
                    buffer[0] = byte;
                    1
                } else {
                    0
                }
            }
        }
    }
}

// Public API
pub use hal::{Relay, Uart};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let relay = Relay::new(registers::RELAY_PIN);
    let mut uart = Uart::new();

    relay.init();
    uart.init();

    loop {
        let mut buffer = [0u8; 1];
        let bytes_read = uart.read(&mut buffer);
        if bytes_read > 0 {
            match buffer[0] {
                b'I' => relay.set_high(), // I for POWER
                b'O' => relay.set_low(),  // O for OFF
                _ => (),                  // IGNORE OTHER BYTES
            }
        }

        // Delay
        for _ in 0..1000 {
            core::hint::spin_loop();
        }
    }
}
