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

    	
	let mut data: Vec<Level> = Vec::new();

    // ask for data and wait for response 
    println!("Request data");

    gpio.write(LTCH_OUT, Level::High);
    while gpio.read(LTCH_IN).unwrap() == Level::Low {}

    println!("Start read");

    // wait for all 24 bits to be received
	for i in 0..24 {
		
        // wait for clock to go from low to high, so we know that the data line is set
		while gpio.read(CLCK).unwrap() == Level::Low {
            //thread::sleep(Duration::from_micros(1));
        }
        // pull the data and store it in an array
		data.push(gpio.read(DS).unwrap());

        // wait for clock to go from high to low, so we know that the sender is done with this bit.
		while gpio.read(CLCK).unwrap() == Level::High {
            //thread::sleep(Duration::from_micros(1));
        }		
	}

    println!("Read finished");

    // after resonse, drop request
    gpio.write(LTCH_OUT, Level::Low);

    println!("Tell sender completed");

    print!("data:");

	for i in 0..data.len() {
		print!("{}", match data[i]{ Level::Low => 0, Level::High => 1, });
	}

    println!("done");
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
