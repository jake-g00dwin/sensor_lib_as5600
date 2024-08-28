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

//#[allow(unused_imports)]
//#[macro_use]
//extern crate alloc;

/*
use embedded_hal::blocking::{
    i2c,
    delay::DelayMs,
};
*/

use embedded_hal::i2c::{I2c, ErrorKind};


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


fn bytes_to_u16(bytes: [u8; 2]) -> u16 {
    let value: u16 = ((bytes[0] as u16) << 8)|((bytes[1] as u16));
    return value;
}

pub struct AS5600<I2C>{
    i2c: I2C,
}

impl <I2C: I2c> AS5600<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_status(&mut self) -> Result<SensorStatus, I2C::Error> {
        let mut buf: [u8; 1] = [0x00];
        
        self.i2c.write_read(
            SENSOR_ADDR,
            &[StatusRegisters::Status as u8],
            &mut buf
        )?;

        Ok(SensorStatus::new(buf[0]))
    }

    pub fn config_start_position(&mut self, start: u16) -> Result<u16, I2C::Error> {
        let data: [u8; 2];
        data = start.to_be_bytes();
        
        let buf: [u8; 3] = [(ConfigRegisters::ZPosHi as u8), data[0], data[1]];
        let mut rbuf: [u8; 2] = [0x00, 0x00];

        self.i2c.write(
            SENSOR_ADDR,
            &buf,
        )?;
        
        self.i2c.write_read(
            SENSOR_ADDR,
            &[ConfigRegisters::ZPosHi as u8],
            &mut rbuf,
        )?;
        
        
        Ok(bytes_to_u16(rbuf))
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
    use embedded_hal::i2c::ErrorKind;
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
        let expectations = [
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![StatusRegisters::Status as u8],
                vec![1<<5]
            ),
        ];

        let i2c = I2cMock::new(&expectations);
       
        let mut sensor = AS5600::new(i2c);
        let status = sensor.read_status();
        assert!(status.is_ok());
        
        let status = status.unwrap();
        assert!(status.is_magnet_detected());
        
        sensor.i2c.done();
    }

    #[test]
    fn read_status_bus_error() {
        let expectations = [
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![StatusRegisters::Status as u8],
                vec![1<<5]
                ).with_error(ErrorKind::Bus),
        ];
        
        let i2c = I2cMock::new(&expectations);

        //SHOULD RETURN AN Result<SensorStatus, I2C::Error>
        //This isn't the case however?
        let mut sensor = AS5600::new(i2c);
        let err = sensor.read_status().unwrap_err();
        assert_eq!(err, ErrorKind::Bus);
        
        sensor.i2c.done();
    }

    #[test]
    fn test_configure_start_position() {
        let expect = [
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(ConfigRegisters::ZPosHi as u8), 0x00, 0x00]
            ),
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(ConfigRegisters::ZPosHi as u8)],
                vec![0x00, 0x00],
            ),
        ];

        let i2c = I2cMock::new(&expect);

        let mut sensor = AS5600::new(i2c);
      
        let start: u16 = 0;
        let result = sensor.config_start_position(start);
        assert_eq!(result.unwrap(), 0);

        sensor.i2c.done();
    }

    #[test]
    fn test_configure_start_max_value() {
        let expect = [
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(ConfigRegisters::ZPosHi as u8), 0x0F, 0xFF]
            ),
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(ConfigRegisters::ZPosHi as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);

        let mut sensor = AS5600::new(i2c);
      
        let start: u16 = 4095;
        let result = sensor.config_start_position(start);
        assert_eq!(result.unwrap(), 4095);

        sensor.i2c.done();
    }

    #[test]
    fn test_config_stop_position() {
        assert!(false);
    }

    #[test]
    fn test_config_angular_range() {
        assert!(false);
    }
}
