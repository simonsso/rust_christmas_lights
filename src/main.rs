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
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Direct);
    let mut d4 = pins.d4.into_output().into_pwm(&timer1);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Direct);

    let mut d0 = pins.d0.into_output().into_pwm(&timer0);
    let mut d1 = pins.d1.into_output().into_pwm(&timer0);

    d0.enable();
    d1.enable();
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

/// Modified Sine table with longer time at 0
const SINE: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3, 4, 5, 5, 6, 7, 9, 10, 11, 12,
    14, 15, 17, 18, 20, 21, 23, 25, 27, 29, 31, 33, 35, 37, 40, 42, 44, 47, 49, 52, 54, 57, 59, 62,
    65, 67, 70, 73, 76, 79, 82, 85, 88, 90, 93, 97, 100, 103, 106, 109, 112, 115, 118, 121, 124,
    128, 131, 134, 137, 140, 143, 146, 149, 152, 155, 158, 162, 165, 167, 170, 173, 176, 179, 182,
    185, 188, 190, 193, 196, 198, 201, 203, 206, 208, 211, 213, 215, 218, 220, 222, 224, 226, 228,
    230, 232, 234, 235, 237, 238, 240, 241, 243, 244, 245, 246, 248, 249, 250, 250, 251, 252, 253,
    253, 254, 254, 254, 253, 253, 252, 251, 250, 250, 249, 248, 246, 245, 244, 243, 241, 240, 238,
    237, 235, 234, 232, 230, 228, 226, 224, 222, 220, 218, 215, 213, 211, 208, 206, 203, 201, 198,
    196, 193, 190, 188, 185, 182, 179, 176, 173, 170, 167, 165, 162, 158, 155, 152, 149, 146, 143,
    140, 137, 134, 131, 128, 124, 121, 118, 115, 112, 109, 106, 103, 100, 97, 93, 90, 88, 85, 82,
    79, 76, 73, 70, 67, 65, 62, 59, 57, 54, 52, 49, 47, 44, 42, 40, 37, 35, 33, 31, 29, 27, 25, 23,
    21, 20, 18, 17, 15, 14, 12, 11, 10, 9, 7, 6, 5, 5, 4, 3, 2, 2, 1, 1, 1, 0,
];
struct Duty {
    internal: i32,
}
impl Duty {
    fn new() -> Self {
        Duty { internal: 0 }
    }
    fn get(&self) -> u8 {
        SINE[(self.internal.abs() as u32).to_le_bytes()[1] as usize]
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
