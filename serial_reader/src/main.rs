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

    let mut tries = 0;

    loop {

        thread::sleep(Duration::from_millis(1000));
        tries += 1;
        println!("Try({}) Request data", tries);

        //let mut data: Vec<Level> = Vec::new();
        let mut bytes = BitVec::from_elem(24, false);

        // ask for data and wait for response     
        if request_data (&mut gpio).is_err() { continue; } 

        if get_serial_byte (&mut gpio, &mut bytes).is_err() { continue; } 

        // after resonse, drop request
        gpio.write(LTCH_OUT, Level::Low);

        println!("Read finished");

        pint_bytes (&bytes);

        println!("");
    }
}

fn request_data (gpio: &mut Gpio) -> Result <(), ()> {
    gpio.write(LTCH_OUT, Level::High);
    match wait_for_pin(&gpio, LTCH_IN, Level::Low) {
        Ok(o) => return Ok(()),
        Err(e) => return Err(()),
    }
}

fn get_serial_bytes (gpio: &mut Gpio, byte: &mut BitVec) -> Result <(), ()> {
    for i in 0..24 {
        if wait_for_pin(&gpio, CLCK, Level::Low).is_err() { return Err(()); }
        if (gpio.read(DS).unwrap() == Level::High) {
            byte.set(i, true);
        } else {
            byte.set(i, false);
        }
        if wait_for_pin(&gpio, CLCK, Level::High).is_err() { return Err(()); }		
    } 
    Ok(())
}

fn pint_bytes (bytes: &BitVec) {
    let cb = bytes.to_bytes().unwrap(); 
    println!("bytes: {}-{}-{}", cb[0], cb[1], cb[2])
}

fn wait_for_pin (gpio: &Gpio, pin: u8, from_state: Level) -> Result <(), ()> {
    let mut timeout = 0;
    while gpio.read(CLCK).unwrap() == from_state && timeout < 10_000_000 {
        timeout += 1;
    }
    if timeout > 9_999_999 {
        Err(())
    } else {
        Ok(())
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
