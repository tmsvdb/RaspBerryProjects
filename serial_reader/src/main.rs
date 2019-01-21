// https://github.com/golemparts/rppal
extern crate rppal;
// http://contain-rs.github.io/bit-vec/bit_vec/
extern crate bit_vec;

use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::iter::Iterator;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;
//use rppal::spi::{Spi, Bus, SlaveSelect, Mode};

use bit_vec::BitVec;

const DS: u8 = 14;
const CLCK: u8 = 15;
const LTCH_IN: u8 = 18;
const LTCH_OUT: u8 = 23; ///Request data;
const TIMEOUT: u32 = 500; // max 999 milliseconds
const STATE_CHANGE_TIME: u64 = 10; // pin value change dampening, in microseconds.

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
        
        // set start values
	    gpio.write(LTCH_OUT, Level::Low);
        tries += 1;
        let mut bits = BitVec::from_elem(24, false);
        thread::sleep(Duration::from_millis(500));
 
        // request new data
        gpio.write(LTCH_OUT, Level::High);

        // wait for request accepted response
        match wait_for_state_change(&gpio, LTCH_IN, Level::Low, Level::High) { 
            Ok(_) => {},
            Err(e) => {
                println!("Request failed: {}", e);
                continue; 
            },
        } 
        
        // gather data -> per bit get data pin state
        for _i in 0..24 {
            // wait for clock to go from high to low, so we know the data line is set
            match wait_for_state_change(&gpio, CLCK, Level::High, Level::Low) { 
                Ok(_) => {},
                Err(e) => {
                    println!("Get data failed: {}", e); 
                    continue; 
                },
            }
            if (gpio.read(DS).unwrap() == Level::High) {
                bits.set(_i, true);
            } else {
                bits.set(_i, false);
            }
        } 

        // wait for request complete
	    match wait_for_state_change(&gpio, LTCH_IN, Level::High, Level::Low) { 
            Ok(_) => {},
            Err(e) => {
                println!("Serial not completed: {}", e); 
                continue;
            },
        }

        // print data
        let cb = bits.to_bytes();
	    println!("try({}) bytes: {}-{}-{}", tries, cb[0], cb[1], cb[2]);
    }
}

fn wait_for_state_change (gpio: &Gpio, pin: u8, from_state: Level, to_state: Level) -> Result <(), String> {

    let mut time = SystemTime::now();

    // wait until pin is in the from_state
    while gpio.read(CLCK).unwrap() != from_state {
        if time.elapsed().unwrap().subsec_millis() >= TIMEOUT { return Err(String::from("start state timeout")) }
    }

    // wait until pin has changed to the to_state
    while gpio.read(CLCK).unwrap() != to_state {
        if time.elapsed().unwrap().subsec_millis() >= TIMEOUT { return Err(String::from("change state timeout")) }
    }

    thread::sleep(Duration::from_micros(STATE_CHANGE_TIME));

    // check the pin state again to after a few microseconds, 
    // to prevent value change stuttering.
    while gpio.read(CLCK).unwrap() != to_state {
        if time.elapsed().unwrap().subsec_millis() >= TIMEOUT { return Err(String::from("stabalize state timeout")) }
    }

    Ok(())
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
