# Development TODO

This is a list of the stuff I need to acomplish for this project.


## Self Development:

- Re-learn Rust.
- Figure out differnces from embedded-hal 0.2.7 and 1.0.0.
- Setup Test suite
- Publish to crates.io


## Project outline

I want to write a i2c library in rust for the AS5600 sensor.

### Requirements

- Good testing(by my standards)
- Full functionality.
- Works with WCH micro-controllers
- Is good enough to be put on my resume.


## LIB main

Main library source file.

*tests*

- Create and destroy as5600 instance.
- Can change the sensor address.
- Can Read the sensor status.


- [ ] Impliment i2c address changing.
- [ ] Impliment the burn setting command.
