//! AS5600 I2C driver:
//!
//! Provides a unit tested driver to access the AHT2X series of sensor modules, 
//! focuses on:
//! 
//! - Providing reliable data.
//! - A safer interface to an i2c sensor.
//! - No infinite loops.
//! - No external dependencies for CRC checksums.
//! - No assumption of reliable hardware(passes back error messages) 
//!



#![cfg_attr(not(test), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

/*
use embedded_hal::blocking::{
    i2c,
    delay::DelayMs,
};
*/

use embedded_hal::i2c::{I2c, Error};


//mod sensor_status;
//pub use crate::sensor_status::SensorStatus;
mod sensor_status;
#[allow(unused_imports)]
pub use crate::sensor_status::SensorStatus;

mod registers;
#[allow(unused_imports)]
pub use crate::registers::{
    ConfigRegisters,
    OutputRegisters,
    StatusRegisters,
    BurnCommands,
};



/// Sensor Address
pub const SENSOR_ADDR: u8 = 0b0011_1010; // = 0x40

pub const STARTUP_DELAY_NS: u8 = 0;
pub const MAX_ATTEMPTS: usize = 3;


pub struct AS5600<I2C>{
    i2c: I2C,
}

impl <I2C: I2c> AS5600<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_status(&mut self) -> Result<SensorStatus, I2C::Error> {
        let mut status = [0];
        self.i2c.write_read(SENSOR_ADDR, &[StatusRegisters::Status as u8], &mut status)?;
        //Ok(status[0])
        Ok(SensorStatus::new(0x00))
    }

    //Example from the embedded-hal 1.0.0 docs
    /*
    pub fn read_temperature(&mut self) -> Result<u8, I2C::Error> {
        let mut temp = [0];
        self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp)?;
        Ok(temp[0])
    }
    */
}


//Impliment functions for the sensor that require the embedded-hal I2C.

// TESTS

#[cfg(test)]
mod sensor_test {
    use embedded_hal_mock::eh1::i2c::{
      Mock as I2cMock,
      Transaction as I2cTransaction,
    };
    
    use super::*;

    //Check that the testing macros are functional.
    #[test]
    fn self_test(){
        assert!(true);
    }

    //Check that Mocking using the embedded_hal_mock crate works
    #[test]
    fn mocking_i2c() {
        let expectations = [
            I2cTransaction::write(SENSOR_ADDR, vec![1, 2]),
            I2cTransaction::read(SENSOR_ADDR, vec![3, 4]),
        ];

        let mut i2c = I2cMock::new(&expectations);
        let mut buf = vec![0u8, 2];

        i2c.write(SENSOR_ADDR, &vec![1, 2]).unwrap();
        i2c.read(SENSOR_ADDR, &mut buf).unwrap();

        assert_eq!(buf, vec![3, 4]);

        i2c.done();
    }

    #[test]
    fn get_status() {
    
    }

}
