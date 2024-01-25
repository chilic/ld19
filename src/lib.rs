use core::time::Duration;
use std::io::Read;
use serial::{unix::TTYPort, SerialPort};

const FRAME_BEGINNING_FLAG: u8 = 0b01010100;
const FRAME_SIZE: usize = 47;
const POINT_PER_PACK: usize = 12;
const SERIAL_PORT_CONF: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::BaudOther(230400),
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

pub struct LD19 {
    port: TTYPort,
}

#[derive(Debug)]
pub struct Point {
    pub distance: u16,
    pub intensity: u8,
}

#[derive(Debug)]
pub struct Frame {
    pub header: u8,
    pub ver_len: u8,
    pub speed: u16,
    pub start_angle: u16,
    pub points: [Point; POINT_PER_PACK],
    pub end_angle: u16,
    pub timestamp: u16,
    pub crc8: u8,
}

pub fn decode(buff: &[u8; FRAME_SIZE]) -> Frame {
    Frame {
        header: u8::from_le_bytes(buff[0..1].try_into().unwrap_or_default()),

        ver_len: u8::from_le_bytes(buff[1..2].try_into().unwrap_or_default()),

        speed: u16::from_le_bytes(buff[2..4].try_into().unwrap_or_default()),

        start_angle: u16::from_le_bytes(buff[4..6].try_into().unwrap_or_default()),

        points: core::array::from_fn(|i| Point {
            distance: u16::from_le_bytes(
                buff[6 + (3 * i)..6 + (3 * i) + 2]
                    .try_into()
                    .unwrap_or_default(),
            ),
            intensity: u8::from_le_bytes(
                buff[6 + 2 + (3 * i)..6 + 2 + (3 * i) + 1]
                    .try_into()
                    .unwrap_or_default(),
            ),
        }),

        end_angle: u16::from_le_bytes(buff[42..44].try_into().unwrap_or_default()),

        timestamp: u16::from_le_bytes(buff[44..46].try_into().unwrap_or_default()),

        crc8: u8::from_le_bytes(buff[46..47].try_into().unwrap_or_default()),
    }
}

impl LD19 {
    pub fn open(&mut self, addres: &str, timeout: Duration) -> Result<(), serial::Error> {
        let mut port = serial::open(addres)?;
        port.configure(&SERIAL_PORT_CONF)?;
        port.set_timeout(timeout)?;

        self.port = port;

        Ok(())
    }

    pub fn read_frame(&mut self) {
        let mut buff = Vec::with_capacity(FRAME_SIZE);

        loop {
            self.port.read_exact(buff.as_mut_slice()).unwrap();

            if buff[0] != FRAME_BEGINNING_FLAG {
                let mut shift: Vec<u8> = Vec::with_capacity(1);
                self.port.read_exact(shift.as_mut_slice()).unwrap();
                continue;
            }

            break;
        }

        // Ok(frame)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
