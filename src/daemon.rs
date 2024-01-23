use std::{
    fs::{self, Permissions},
    io::{Read, Write},
    os::unix::{
        fs::PermissionsExt,
        net::{UnixListener, UnixStream},
    },
    sync::{Arc, Mutex},
    thread,
};

use pico_gpio::{
    serialport::{self, Error, SerialPort},
    PicoGPIO,
};

use crate::*;

pub fn start(file: &str, baud: u32, socket_file: &str) {
    let mut port = serialport::new(file, baud)
        .open_native()
        .expect("unable to open serial port");
    port.set_exclusive(true).unwrap();
    let gpio = PicoGPIO::new(port).expect("unable to open PicoGPIO connection");

    let gpio = Arc::new(Mutex::new(gpio));
    let _ = fs::remove_file(socket_file);
    let listener = UnixListener::bind(socket_file).expect("unable to start socket");

    fs::set_permissions(socket_file, Permissions::from_mode(0o666))
        .expect("unable to make connection available");

    for connection in listener.incoming() {
        let Ok(connection) = connection else {
            continue;
        };
        let gpio = gpio.clone();
        thread::spawn(move || {
            let _ = handle(connection, gpio);
        });
    }
}

fn handle<T: SerialPort>(
    mut connection: UnixStream,
    gpio: Arc<Mutex<PicoGPIO<T>>>,
) -> Result<(), Error> {
    connection.write_all(&[1])?;
    let mut buf1 = [0u8; 1];
    let mut buf2 = [0u8; 2];

    loop {
        connection.read_exact(&mut buf1)?;
        match buf1[0] {
            DISCONNECT => break,
            NOP => continue,
            _ => (),
        };
        connection.read_exact(&mut buf2)?;
        let pin = u16::from_be_bytes(buf2) as usize;
        let mut gpio = gpio.lock().unwrap();
        match buf1[0] {
            READ_FLOATING => response_digital(&mut connection, gpio.in_float(pin)?)?,
            READ_PULLUP => response_digital(&mut connection, gpio.in_pullup(pin)?)?,
            READ_PULLDN => response_digital(&mut connection, gpio.in_pulldn(pin)?)?,
            READ_ANALOG => response_analog(&mut connection, gpio.in_analog(pin)?)?,
            WRITE_FLOATING => gpio.float(pin)?,
            WRITE_DIGITAL => {
                connection.read_exact(&mut buf1)?;
                gpio.out_d(pin, buf1[0] != 0)?;
            }
            WRITE_PWM => {
                connection.read_exact(&mut buf2)?;
                gpio.out_pwm(pin, u16::from_be_bytes(buf2) as u32)?;
            }
            _ => break,
        }
        connection.write_all(&[1])?;
    }

    Ok(())
}

fn response_digital(connection: &mut UnixStream, val: bool) -> Result<(), Error> {
    connection.write_all(&[if val { 1 } else { 0 }])?;
    Ok(())
}
fn response_analog(connection: &mut UnixStream, val: u32) -> Result<(), Error> {
    connection.write_all(&(val as u16).to_be_bytes())?;
    Ok(())
}
