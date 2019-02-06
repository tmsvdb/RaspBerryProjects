extern crate rppal;
extern crate bit_vec;

use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::iter::Iterator;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

use bit_vec::BitVec;

// serial reader setings
const R_DS: u8 = 14;
const R_CLCK: u8 = 15;
const R_LTCH_IN: u8 = 18;
const R_LTCH_OUT: u8 = 23; ///Request data;
const TIMEOUT: u32 = 500; // max 999 milliseconds
const STATE_CHANGE_TIME: u64 = 10; // pin value change dampening, in microseconds.

// serial stepper settings
const W_DS: u8 = 2;
const W_CLCK: u8 = 3;
const W_LTCH: u8 = 4;


fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();

    gpio.set_mode(R_DS, Mode::Input);
    gpio.set_mode(R_CLCK, Mode::Input);
    gpio.set_mode(R_LTCH_IN, Mode::Input);
    gpio.set_mode(R_LTCH_OUT, Mode::Output);

    gpio.write(R_LTCH_OUT, Level::Low);



    let (tx, rx) = mpsc::channel();

    let read = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));     
        let cb = read_data (&gpio).to_bytes();      
        tx.send(cb).unwrap();
    });


    loop {

        match rx.try_recv() {
            Ok(cb) => { println!("bytes: {}-{}-{}", cb[0], cb[1], cb[2]); },
            Err() => {},
        }

        /*
        for _n in 0..4096 {
            motors.motor_1(MotorAction::RotateCW);
            motors.motor_2(MotorAction::RotateCW);
            motors.motor_3(MotorAction::RotateCW);
            write_motors (&gpio, &motors.translate_all());
            thread::sleep(Duration::from_millis(1));
        }*/
    }
}


// SERIAL READER HELPERS
// =====================

fn read_data (gpio: &Gpio) -> BitVec {

    // set start values   
    let mut bits = BitVec::from_elem(24, false);
    
    // request new data
    gpio.write(R_LTCH_OUT, Level::High);

    // wait for request accepted response
    match wait_for_state_change(&gpio, R_LTCH_IN, Level::Low, Level::High) { 
        Ok(_) => {},
        Err(e) => {
            println!("Request failed: {}", e);
            continue; 
        },
    } 
    
    // gather data -> per bit get data pin state
    for _i in 0..24 {
        // wait for clock to go from high to low, so we know the data line is set
        match wait_for_state_change(&gpio, R_CLCK, Level::High, Level::Low) { 
            Ok(_) => {},
            Err(e) => {
                println!("Get data failed: {}", e); 
                continue; 
            },
        }
        if (gpio.read(R_DS).unwrap() == Level::High) {
            bits.set(_i, true);
        } else {
            bits.set(_i, false);
        }
    } 

    // wait for request complete
    match wait_for_state_change(&gpio, R_LTCH_IN, Level::Low, Level::Low) { 
        Ok(_) => {},
        Err(e) => {
            println!("Serial not completed: {}", e); 
            continue;
        },
    }

    gpio.write(R_LTCH_OUT, Level::Low);
    bits
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


// SERIAL WRITE HELPERS
// ====================

fn write_motors (gpio: &Gpio, data: &Vec<bool>) {
    gpio.write(W_LTCH, Level::Low); 
    for _i in 0..16 {
        gpio.write(W_DS, match data[_i] { true => Level::High, false => Level::Low, } ); 
        gpio.write(W_CLCK, Level::High); 
        gpio.write(W_CLCK, Level::Low); 
    }
    gpio.write(W_LTCH, Level::High); 
}

enum MotorAction { RotateCW, RotateCCW, ShutDown }

struct MotorStates {
    state_1: i8,
    state_2: i8,
    state_3: i8,
}

impl MotorStates {

    pub fn motor_1 (&mut self, action: MotorAction) {

        match action {
            MotorAction::RotateCW => { self.state_1 += 1; if self.state_1 > 7 { self.state_1 = 0; }},
            MotorAction::RotateCCW => { self.state_1 -= 1; if self.state_1 < 0 { self.state_1 = 7; }},
            MotorAction::ShutDown => self.state_1 = -1,
        }
    }

    pub fn motor_2 (&mut self, action: MotorAction) {

        match action {
            MotorAction::RotateCW => { self.state_2 += 1; if self.state_2 > 7 { self.state_2 = 0; }},
            MotorAction::RotateCCW => { self.state_2 -= 1; if self.state_2 < 0 { self.state_2 = 7; }},
            MotorAction::ShutDown => self.state_2 = -1,
        }
    }

    pub fn motor_3 (&mut self, action: MotorAction) {

        match action {
            MotorAction::RotateCW => { self.state_3 += 1; if self.state_3 > 7 { self.state_3 = 0; }},
            MotorAction::RotateCCW => { self.state_3 -= 1; if self.state_3 < 0 { self.state_3 = 7; }},
            MotorAction::ShutDown => self.state_3 = -1,
        }
    }

    fn translate_all (&mut self) -> Vec<bool> {
        let s1 = self.translate(&self.state_1);
        let s2 = self.translate(&self.state_2);
        let s3 = self.translate(&self.state_3);
        
        vec![   s1[0], s1[1], s1[2], s1[3], 
                s2[0], s2[1], s2[2], s2[3], 
                s3[0], s3[1], s3[2], s3[3],
                false, false, false, false]
    }

    fn translate (&self, state: &i8) -> Vec<bool> {
        match state {
            0 => vec![false, false, false, true],
            1 => vec![false, false, true, true],
            2 => vec![false, false, true, false],
            3 => vec![false, true, true, false],
            4 => vec![false, true, false, false],
            5 => vec![true, true, false, false],
            6 => vec![true, false, false, false],
            7 => vec![true, false, false, true],
            _ => vec![false, false, false, false],
        }
    }
}
