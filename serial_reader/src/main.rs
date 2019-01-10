// https://github.com/golemparts/rppal
extern crate rppal;
// http://contain-rs.github.io/bit-vec/bit_vec/
extern crate bit_vec;

use std::thread;
use std::time::Duration;
use std::iter::Iterator;

//use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;
use rppal::spi::{Spi, Bus, SlaveSelect, Mode};

use bit_vec::BitVec;

const DS: u8 = 10;
const CLCK: u8 = 11;
const LTCH: u8 = 8;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());
/*
    let mut gpio = Gpio::new().unwrap();

    gpio.set_mode(DS, Mode::Input);
    gpio.set_mode(CLCK, Mode::Output);
    gpio.set_mode(LTCH, Mode::Output);
*/
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();

    loop {
        let mut buffer: [u8; 2] = [0; 2];
        let size = spi.read(&mut buffer).unwrap();
        println!("b:{}, s:{}", buffer[0], size);
    }
}
/*
fn read (gpio: &Gpio) -> u8 {
    let mut data = BitVec::from_elem(8, false);
    gpio.write(LTCH, Level::Low); 
    for _i in 0..8 {
        gpio.write(CLCK, Level::High); 
        gpio.write(CLCK, Level::Low); 
    }
    thread::sleep(Duration::from_millis(1));
    for _i in 0..8 {
        gpio.write(CLCK, Level::High); 
        gpio.write(CLCK, Level::Low); 
    }
    thread::sleep(Duration::from_millis(1));
    for _i in 0..8 {
        match gpio.read(DS).unwrap() { 
            Level::High => data.set(_i, true), 
            Level::Low => data.set(_i, false), 
        };
        gpio.write(CLCK, Level::High); 
        gpio.write(CLCK, Level::Low); 
    }
    gpio.write(LTCH, Level::High);
    thread::sleep(Duration::from_millis(1));
    data.to_bytes()[0] 
}
*/