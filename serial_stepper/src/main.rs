extern crate rppal;

use std::thread;
use std::time::Duration;
use std::iter::Iterator;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

const DS: u8 = 2;
const CLCK: u8 = 3;
const LTCH: u8 = 4;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();

    gpio.set_mode(DS, Mode::Output);
    gpio.set_mode(CLCK, Mode::Output);
    gpio.set_mode(LTCH, Mode::Output);

    let mut motors = MotorStates { state_1: 0, state_2: 0, state_3: 0};

    for _n in 0..4096 {
        motors.motor_1(MotorAction::RotateCW);
        motors.motor_2(MotorAction::RotateCW);
        motors.motor_3(MotorAction::RotateCW);
        write (&gpio, &motors.translate_all());
        thread::sleep(Duration::from_millis(1));
    }

    write (&gpio, &vec![false,false,false,false, false,false,false,false, false,false,false,false, false,false,false,false]);
}

fn write (gpio: &Gpio, data: &Vec<bool>) {
    gpio.write(LTCH, Level::Low); 
    for _i in 0..16 {
        gpio.write(DS, match data[_i] { true=> Level::High, false => Level::Low, } ); 
        gpio.write(CLCK, Level::High); 
        gpio.write(CLCK, Level::Low); 
    }
    gpio.write(LTCH, Level::High); 
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