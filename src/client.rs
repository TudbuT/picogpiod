use std::{
    io::{self, Read, Write},
    os::unix::net::UnixStream,
};

use crate::*;

pub struct Client {
    socket: UnixStream,
}

impl Client {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        Ok(Self {
            socket: UnixStream::connect(path)?,
        })
    }

    pub fn disconnect(mut self) {
        let _ = self.socket.write_all(&[DISCONNECT]);
    }

    fn ensure_alive(&mut self) -> Result<(), io::Error> {
        let mut buf = [0u8; 1];
        self.socket.read_exact(&mut buf)?;
        if buf[0] != 1 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data in pgpio socket",
            ))
        } else {
            Ok(())
        }
    }

    pub fn read_floating(&mut self, pin: u16) -> Result<bool, io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[READ_FLOATING])?;
        self.do_read(pin)
    }

    pub fn read_pullup(&mut self, pin: u16) -> Result<bool, io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[READ_PULLUP])?;
        self.do_read(pin)
    }

    pub fn read_pulldn(&mut self, pin: u16) -> Result<bool, io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[READ_PULLDN])?;
        self.do_read(pin)
    }

    fn do_read(&mut self, pin: u16) -> Result<bool, io::Error> {
        self.socket.write_all(&pin.to_be_bytes())?;
        let mut buf = [0u8; 1];
        self.socket.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }

    pub fn read_analog(&mut self, pin: u16) -> Result<u16, io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[READ_ANALOG])?;
        self.socket.write_all(&pin.to_be_bytes())?;
        let mut buf = [0u8; 2];
        self.socket.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn float(&mut self, pin: u16) -> Result<(), io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[WRITE_FLOATING])?;
        self.socket.write_all(&pin.to_be_bytes())?;
        Ok(())
    }

    pub fn write_digital(&mut self, pin: u16, val: bool) -> Result<(), io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[WRITE_DIGITAL])?;
        self.socket.write_all(&pin.to_be_bytes())?;
        self.socket.write_all(&[if val { 1 } else { 0 }])?;
        Ok(())
    }

    pub fn write_pwm(&mut self, pin: u16, val: u16) -> Result<(), io::Error> {
        self.ensure_alive()?;
        self.socket.write_all(&[WRITE_DIGITAL])?;
        self.socket.write_all(&pin.to_be_bytes())?;
        self.socket.write_all(&val.to_be_bytes())?;
        Ok(())
    }
}
