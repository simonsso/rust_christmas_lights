## Digispark ATtiny85 Christmas light

This is a rust reimplementaion for https://github.com/simonsso/christmas_lights targeting only the Digispark ATtiny low cost hardware platform.

This project is created using [Tomasz Wiszkowski's templates](https://github.com/tomasz-wiszkowski/rust-attiny85-blink)
### Requirements

To use this, you need to 
* download  `micronucleus` from [Github repo](https://github.com/micronucleus/micronucleus/tree/master). 
* install Rust nightly from 2022
* install AVR GCC

```
rustup toolchain add nightly-2022-07-10
rustup component add rust-src --toolchain nightly-2022-07-10-x86_64-unknown-linux-gnu
sudo apt-get install avr-libc gcc-avr
curl https://raw.githubusercontent.com/micronucleus/micronucleus/refs/heads/master/commandline/49-micronucleus.rules > 49-micronucleus.rules
sudo mv 49-micronucleus.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

### ATTiny85 notes

Due to limitations of the programmable memory on this microcontroller,
* be sure to build and run `---release` version (`cargo build --release` or `cargo run --release`) - otherwise your code just won't fit.
* the code interfaces registers directly to preserve programmable memory (believe it or not, it's also faster this way)

### Other notes
* This has only been used on Linux. Not sure how this will work on other platforms.
* When using Arduino IDE, replace the Digispark-shipped `micronucleus` with the one you built. 
* `micronucleus` repo has bootrom upgrades that can be used to update your `attiny85`
* Updated `attiny85` won't work with obsolete version shipped by Digispark.

### Build run and flash
```
DEVICE=attiny85 cargo +nightly-2022-07-10 run --release
```
