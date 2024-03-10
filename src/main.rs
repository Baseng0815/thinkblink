use std::{time::Duration, fs::{File, OpenOptions}, io::{self, Write, stdin}, thread};

use morse;

fn blink(duration: Duration) {
    let mut file = OpenOptions::new().write(true).open("/sys/devices/platform/thinkpad_acpi/leds/tpacpi::lid_logo_dot/brightness").unwrap();
    file.write_all(b"1").unwrap();
    thread::sleep(duration);
    file.write_all(b"0").unwrap();
}

fn transmit(message: &str) {
    let encoded = morse::encode::encode(message).unwrap();
    let dot = Duration::from_millis(100);

    for symbol in encoded.chars() {
        match symbol {
            '.' => blink(dot), // 1 in total
            '_' => blink(3 * dot), // 3 in total
            ' ' => thread::sleep(2 * dot), // 3 in total
            '/' => thread::sleep(dot), // 7 in total
            _ => panic!("Invalid character in encoded message")
        }

        thread::sleep(dot);
    }
}

fn main()  {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        transmit(&line);
    }
}
