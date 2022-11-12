#![no_std]
#![no_main]

const INTERNAL_MAX: i32 = 0xFF00;
extern crate panic_halt;

use arduino_hal::simple_pwm::*;
use core::ops;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let mut led = pins.d1.into_output();
    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Direct);
    let mut d4 = pins.d4.into_output().into_pwm(&timer1);

    // led.set_low();
    // arduino_hal::delay_ms(2000);

    // led.set_high();
    //

    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Direct);

    let mut d0 = pins.d0.into_output().into_pwm(&timer0);
    let mut d1 = pins.d1.into_output().into_pwm(&timer0);

    d0.set_duty(0);
    d0.enable();
    d1.set_duty(0);
    d1.enable();
    d4.set_duty(0);
    d4.enable();

    let mut duty_0 = Duty::new();
    let mut duty_1 = Duty::new();
    let mut duty_4 = Duty::new();
    loop {
        d0.set_duty(duty_0.get());
        d1.set_duty(duty_1.get());
        d4.set_duty(duty_4.get());

        duty_0 += 29;
        duty_1 += 13;
        duty_4 += 37;
        arduino_hal::delay_ms(20);
    }
}

struct Duty {
    internal: i32,
}
impl Duty {
    fn new() -> Self {
        Duty { internal: 0 }
    }
    fn get(&self) -> u8 {
        (self.internal.abs() as u32).to_le_bytes()[1] / 4
    }
}

impl ops::AddAssign<i32> for Duty {
    fn add_assign(&mut self, incr: i32) {
        if self.internal + incr > INTERNAL_MAX {
            self.internal = -self.internal;
        }
        self.internal = self.internal + incr;
    }
}