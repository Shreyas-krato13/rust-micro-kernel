use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::instructions::port::Port;

static SHIFT: AtomicBool = AtomicBool::new(false);

// CONVERT SCANCODE TO ASCII

fn scancode_to_ascii(code: u8, shift: bool) -> Option<u8> {
    let character = match code {
        0x02 => if shift { b'!' } else { b'1' },
        0x03 => if shift { b'@' } else { b'2' },
        0x04 => if shift { b'#' } else { b'3' },
        0x05 => if shift { b'$' } else { b'4' },
        0x06 => if shift { b'%' } else { b'5' },
        0x07 => if shift { b'^' } else { b'6' },
        0x08 => if shift { b'&' } else { b'7' },
        0x09 => if shift { b'*' } else { b'8' },
        0x0A => if shift { b'(' } else { b'9' },
        0x0B => if shift { b')' } else { b'0' },

        0x10 => if shift { b'Q' } else { b'q' },
        0x11 => if shift { b'W' } else { b'w' },
        0x12 => if shift { b'E' } else { b'e' },
        0x13 => if shift { b'R' } else { b'r' },
        0x14 => if shift { b'T' } else { b't' },
        0x15 => if shift { b'Y' } else { b'y' },
        0x16 => if shift { b'U' } else { b'u' },
        0x17 => if shift { b'I' } else { b'i' },
        0x18 => if shift { b'O' } else { b'o' },
        0x19 => if shift { b'P' } else { b'p' },

        0x1E => if shift { b'A' } else { b'a' },
        0x1F => if shift { b'S' } else { b's' },
        0x20 => if shift { b'D' } else { b'd' },
        0x21 => if shift { b'F' } else { b'f' },
        0x22 => if shift { b'G' } else { b'g' },
        0x23 => if shift { b'H' } else { b'h' },
        0x24 => if shift { b'J' } else { b'j' },
        0x25 => if shift { b'K' } else { b'k' },
        0x26 => if shift { b'L' } else { b'l' },

        0x2C => if shift { b'Z' } else { b'z' },
        0x2D => if shift { b'X' } else { b'x' },
        0x2E => if shift { b'C' } else { b'c' },
        0x2F => if shift { b'V' } else { b'v' },
        0x30 => if shift { b'B' } else { b'b' },
        0x31 => if shift { b'N' } else { b'n' },
        0x32 => if shift { b'M' } else { b'm' },

        0x39 => b' ',
        _ => return None,
    };

    Some(character)
}

// KEYBOARD POLLING

pub fn poll() {
    let mut status_port = Port::<u8>::new(0x64);
    let mut data_port = Port::<u8>::new(0x60);

    unsafe {
        if status_port.read() & 1 == 0 {
            return;
        }

        let code = data_port.read();

        // SHIFT PRESSED
        if code == 0x2A || code == 0x36 {
            SHIFT.store(true, Ordering::Relaxed);
            return;
        }

        // SHIFT RELEASED
        if code == 0xAA || code == 0xB6 {
            SHIFT.store(false, Ordering::Relaxed);
            return;
        }

        // IGNORE KEY RELEASES
        if code & 0x80 != 0 {
            return;
        }

        // ENTER KEY
        if code == 0x1C {
            crate::kprintln!("");
            return;
        }

        // BACKSPACE KEY
if code == 0x0E {
    crate::vga_buffer::WRITER.lock().delete_char();
    return;
}
        let shift = SHIFT.load(Ordering::Relaxed);

        if let Some(character) = scancode_to_ascii(code, shift) {
            crate::kprint!("{}", character as char);
        }
    }
}
