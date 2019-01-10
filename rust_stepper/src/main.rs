extern crate rppal;

use std::thread;
use std::time::Duration;
use std::iter::Iterator;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut c = StepperController::new();
        c.register_stepper(Stepper {null_pin: 6, pin1: 2, pin2: 3, pin3: 4, pin4: 5});

/*
    // offset stepper:
    // ===============

    while controller.total_steps < 100 {
        controller.step(Direction::CW);
        thread::sleep(Duration::from_millis(1));
    }
*/
    c.find_null (0, Direction::CCW, 20);
    c.multi_step(0, Direction::CW, 0, 1024);
}

enum Direction { CW, CCW  }

struct Stepper {
    null_pin: u8, // read null position
    pin1: u8, // write stepper pin 1
    pin2: u8, // write stepper pin 2
    pin3: u8, // write stepper pin 3
    pin4: u8, // write stepper pin 4
}

struct StepperController {
    gpio: Gpio,
    steppers:Vec<Stepper>,
    sub_step: u8,
    total_steps: u64,
}

impl StepperController {

    fn new () -> StepperController {   
        StepperController {
            gpio: Gpio::new().unwrap(),
            steppers: Vec::new(),
    	    sub_step: 7,
            total_steps: 0,
	    }
    }

    fn register_stepper (&mut self, stepper: Stepper) {
        self.gpio.set_mode(stepper.null_pin, Mode::Input);
        self.gpio.set_mode(stepper.pin1, Mode::Output);
        self.gpio.set_mode(stepper.pin2, Mode::Output);
        self.gpio.set_mode(stepper.pin3, Mode::Output);
        self.gpio.set_mode(stepper.pin4, Mode::Output);
        self.steppers.push(stepper);
    }

    fn set_pins (&self, stepper: usize, pl1: Level, pl2: Level, pl3: Level, pl4: Level) {
        let s = &self.steppers[stepper];
        self.gpio.write(s.pin1, pl1);  
        self.gpio.write(s.pin2, pl2); 
        self.gpio.write(s.pin3, pl3); 
        self.gpio.write(s.pin4, pl4);
        //println!("write -> pin{}:{}, pin{}:{}, pin{},{}, pin{}:{}", s.pin1, pl1, s.pin2, pl2, s.pin3, pl3, s.pin4, pl4);
    }

    fn reset_pins (&mut self, stepper: usize) {
        self.set_pins(stepper, Level::Low, Level::Low, Level::Low, Level::Low);
        self.sub_step = 7;
    }

    fn step (&mut self, stepper: usize, direction: &Direction) {     
        self.sub_step = match direction {
            Direction::CW => if self.sub_step < 7 { self.sub_step + 1 } else { 0 },
            Direction::CCW => if self.sub_step > 0 { self.sub_step - 1 } else { 7 },
        };
        self.total_steps += 1;
        match self.sub_step { 
            0 => { self.set_pins(stepper, Level::Low, Level::Low, Level::Low, Level::High); },
            1 => { self.set_pins(stepper, Level::Low, Level::Low, Level::High, Level::High); },
            2 => { self.set_pins(stepper, Level::Low, Level::Low, Level::High, Level::Low); }, 
            3 => { self.set_pins(stepper, Level::Low, Level::High, Level::High, Level::Low); }, 
            4 => { self.set_pins(stepper, Level::Low, Level::High, Level::Low, Level::Low); }, 
            5 => { self.set_pins(stepper, Level::High, Level::High, Level::Low, Level::Low); }, 
            6 => { self.set_pins(stepper, Level::High, Level::Low, Level::Low, Level::Low); },
            7 => { self.set_pins(stepper, Level::High, Level::Low, Level::Low, Level::High); }, 
            _ => { self.set_pins(stepper, Level::Low, Level::Low, Level::Low, Level::Low); },  
        }
    }

    fn multi_step (
        &mut self, 
        stepper: usize,
        direction: Direction, 
        delay_per_step: u64, 
        num_steps: u64, 
    ) {
        while self.total_steps < num_steps {
            self.step(stepper, &direction);
            thread::sleep(Duration::from_millis(1 + delay_per_step));
        }
        self.total_steps = 0;
        self.reset_pins (stepper);
        println!("complete");
    }

    fn find_null (&mut self, stepper: usize, direction: Direction, delay_per_step: u64) {
        while self.gpio.read(self.steppers[stepper].null_pin).unwrap() == Level::Low {
            self.step(stepper, &direction);
            thread::sleep(Duration::from_millis(1 + delay_per_step));
        }
        self.total_steps = 0;
        self.reset_pins (stepper);
    }
}
