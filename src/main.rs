#![no_std]
#![no_main]

const INTERNAL_MAX: i32 = 0xFF00;
extern crate panic_halt;

use core::ops;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut d4 = pins.d4.into_output();

    // let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Direct);

    let mut d0 = pins.d0.into_output();
    let mut d1 = pins.d1.into_output();

    d0.set_low();
    d1.set_low();
    d4.set_low();

    let mut duty_0 = Duty::new();
    let mut duty_1 = Duty::new();
    let mut duty_4 = Duty::new();
    loop {
        let mut d0_pwm_duty = Deposit::new(duty_0.get());
        let mut d1_pwm_duty = Deposit::new(duty_1.get());
        let mut d4_pwm_duty = Deposit::new(duty_4.get());
        d0.set_high();
        d1.set_high();
        d4.set_high();
        for _ in 0..256 {
            if Some(true) == d0_pwm_duty.next() {
                d0.set_low();
            }
            if Some(true) == d1_pwm_duty.next() {
                d1.set_low();
            }
            if Some(true) == d4_pwm_duty.next() {
                d4.set_low();
            }
            arduino_hal::delay_us(30);
        }
        arduino_hal::delay_ms(2);

        // d1.toggle();

        duty_0 += 29;
        duty_1 += 13;
        duty_4 += 37;
    }
}
#[derive(Default)]
struct Deposit {
    times: Option<u8>,
}

impl Deposit {
    fn new(t: u8) -> Self {
        Deposit { times: Some(t) }
    }
}
impl Iterator for Deposit {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(times) = self.times {
            if times == 0 {
                self.times = None;
                Some(true)
            } else {
                self.times = Some(times.saturating_sub(1));
                Some(false)
            }
        } else {
            None
        }
    }
}

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
