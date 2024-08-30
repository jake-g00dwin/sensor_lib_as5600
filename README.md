# sensor_lib_as5600
A Rust sensor library for the AS5600 magnetic rotary encoder module.

## Current State

The library has partial functionality at this point. It's targeting the new
version of the embedded-hal.

It should be able to read and configure most the registers for the device; 
however burning and changing the I2C functionality isn't yet implimented.

## Usage

### Adding to project

There are two ways to add the crate/repo to your project. You can use the 
github URL or you can add it via the normal crates name(TBD).

### Running Tests

To run the tests for the project after downloading or cloning the repo

```sh
cargo test
```

## Roadmap

- [ ] Define all Bitmasks for registers.
- [ ] Add non-default i2c addressing.
- [ ] setup un-initialized and initialized versions of the sensor.
- [ ] Add example usage.
- [ ] Create an example repo for drop-in uC examples.
- [ ] Create Comperhensive Documentation using D.S. images.
- [ ] Create and link to example usage video.


## Contributing

If you want to contribute to it feel free to open up a pull-request or make a 
new github issue.


