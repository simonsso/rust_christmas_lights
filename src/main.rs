#![no_std]
#![no_main]
#![feature(int_roundings)]

const INTERNAL_MAX: i32 = 0xFF00;
extern crate panic_halt;

use core::ops;

use deposit_iter::Deposit;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut d0 = pins.d0.into_output();
    let mut d1 = pins.d1.into_output();
    let mut d2 = pins.d2.into_output();
    let mut d3 = pins.d3.into_output();
    let mut d4 = pins.d4.into_output();

    d0.set_low();
    d1.set_low();
    d2.set_low();
    d3.set_low();
    d4.set_low();

    let mut duty_0 = Duty::new();
    let mut duty_1 = Duty::new();
    let mut duty_2 = Duty::new();
    let mut duty_3 = Duty::new();
    let mut duty_4 = Duty::new();

    loop {
        let mut d0_pwm_duty = Deposit::new(duty_0.get().into());
        let mut d1_pwm_duty = Deposit::new(duty_1.get().into());
        let mut d2_pwm_duty = Deposit::new(duty_2.get().into());
        let mut d3_pwm_duty = Deposit::new(duty_3.get().into());
        let mut d4_pwm_duty = Deposit::new(duty_4.get().into());
        d0.set_high();
        d1.set_high();
        d2.set_high();
        d3.set_high();
        d4.set_high();
        for _ in 0..256 {
            if Some(true) == d0_pwm_duty.next() {
                d0.set_low();
            }
            if Some(true) == d1_pwm_duty.next() {
                d1.set_low();
            }
            if Some(true) == d2_pwm_duty.next() {
                d2.set_low();
            }
            if Some(true) == d3_pwm_duty.next() {
                d3.set_low();
            }
            if Some(true) == d4_pwm_duty.next() {
                d4.set_low();
            }
            arduino_hal::delay_us(30);
        }
        arduino_hal::delay_ms(2);

        duty_0 += 29;
        duty_1 += 13;
        duty_2 += 17;
        duty_3 += 23;
        duty_4 += 37;
    }
}
#[derive(Default)]

struct Duty {
    internal: i32,
}
impl Duty {
    fn new() -> Self {
        Duty {
            internal: -INTERNAL_MAX,
        }
    }
    fn get(&self) -> u8 {
        // SINE[(self.internal.abs() as u32).to_le_bytes()[1] as usize]
        (self.internal.abs() as u32).to_le_bytes()[1] / 2
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
