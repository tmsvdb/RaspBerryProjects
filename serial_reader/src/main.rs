// https://github.com/golemparts/rppal
extern crate rppal;
// http://contain-rs.github.io/bit-vec/bit_vec/
extern crate bit_vec;

use std::thread;
use std::time::Duration;
use std::iter::Iterator;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;
//use rppal::spi::{Spi, Bus, SlaveSelect, Mode};

use bit_vec::BitVec;

const DS: u8 = 10;
const CLCK: u8 = 11;
const LTCH_IN: u8 = 8;
const LTCH_OUT: u8 = 7; ///Request data;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();

    gpio.set_mode(DS, Mode::Input);
    gpio.set_mode(CLCK, Mode::Input);
    gpio.set_mode(LTCH_IN, Mode::Input);
    gpio.set_mode(LTCH_OUT, Mode::Output);

    loop {

        //let mut data: Vec<Level> = Vec::new();
        let mut byte_1 = BitVec::from_elem(8, false);
        let mut byte_2 = BitVec::from_elem(8, false);
        let mut byte_3 = BitVec::from_elem(8, false);

        // ask for data and wait for response     
        request_data (&mut gpio);

        get_serial_byte (&mut gpio, &mut byte_1);
        get_serial_byte (&mut gpio, &mut byte_2);
        get_serial_byte (&mut gpio, &mut byte_3);

        // after resonse, drop request
        gpio.write(LTCH_OUT, Level::Low);

        println!("Read finished");

        pint_byte (&byte_1);
        pint_byte (&byte_2);
        pint_byte (&byte_3);

        println!("");

        thread::sleep(Duration::from_millis(1000));
    }
}

fn request_data (gpio: &mut Gpio) {
    println!("Try Request data");
    let timeout = 0;
    while gpio.read(LTCH_IN).unwrap() == Level::Low && timeout < 10000 {
        gpio.write(LTCH_OUT, Level::High);
        timeout += 1;
    }
    if timeout > 9999 {
        gpio.write(LTCH_OUT, Level::Low);
        thread::sleep(Duration::from_millis(100));
        request_data (gpio);
    }
}

fn get_serial_byte (gpio: &mut Gpio, byte: &mut BitVec) {
    for i in 0..8 {
        while gpio.read(CLCK).unwrap() == Level::Low {}
        byte.set(3, gpio.read(DS).unwrap() == Level::High);
        while gpio.read(CLCK).unwrap() == Level::High {}		
    } 
}

fn pint_byte (byte: &BitVec) {
    print!("[");
    for i in 0..8 {
        print!("{}", match byte.get(i).unwrap_or(false) { false => 0, true => 1, });
    }
    print!("]");
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
