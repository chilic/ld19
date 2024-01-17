extern crate serial;

use bincode::deserialize;
use serde::Deserialize;
use std::env;
use std::mem;
use std::time::Duration;

// use std::io::prelude::*;
use serial::prelude::*;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::BaudOther(230400),
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

const POINT_PER_PACK: usize = 12;
const FRAME_SIZE: usize = mem::size_of::<Frame>();

#[repr(C, packed)]
#[derive(Debug, Deserialize, Copy, Clone)]
struct Point {
    distance: u16,
    intensity: u8,
}

#[repr(C, packed)]
#[derive(Debug, Deserialize, Copy, Clone)]
struct Frame {
    header: u8,
    ver_len: u8,
    speed: u16,
    start_angle: u16,
    points: [Point; POINT_PER_PACK],
    end_angle: u16,
    timestamp: u16,
    crc8: u8,
}

fn main() {
    for arg in env::args_os().skip(1) {
        println!("opening port: {:?}", arg);
        let mut port = serial::open(&arg).unwrap();

        interact(&mut port).unwrap();
    }
}

fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    port.configure(&SETTINGS)?;
    port.set_timeout(Duration::from_millis(10))?;

    loop {
        let mut buf: Vec<u8> = vec![0; FRAME_SIZE];
        port.read_exact(&mut buf[..])?;

        if buf[0] != 0b01010100 || buf[1] != 0b00101100 {
            let mut correction: Vec<u8> = vec![0; 1];
            port.read_exact(&mut correction[..])?;
            println!("correction");
            continue;
        }

        let foobar: Frame = deserialize(&buf).unwrap();

        if foobar.start_angle > 0 && foobar.end_angle < 1 * 100 {
            for point in foobar.points.into_iter() {
                if point.intensity > 0 {
                    println!("{:?}", point);
                    // println!("{:?}", foobar);
                    break;

                    // let unaligned = std::ptr::addr_of!(foobar.start_angle);
                    // let y = unsafe { std::ptr::read_unaligned(unaligned) };
                    // println!("{:?}", y);
                }
            }
        }
    }
    // Ok(())
}
