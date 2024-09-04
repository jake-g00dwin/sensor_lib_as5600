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
        self.write_12bits(ConfigRegisters::ZPosHi as u8, start)?;
        let verify_value = self.read_12bits(ConfigRegisters::ZPosHi as u8)?;
        Ok(verify_value)
    }

    pub fn config_stop_position(&mut self, stop: u16) -> Result<u16, I2C::Error> {
        self.write_12bits(ConfigRegisters::MPosHi as u8, stop)?;
        let verify_value = self.read_12bits(ConfigRegisters::MPosHi as u8)?;
        Ok(verify_value)
    }

    pub fn config_angular_range(&mut self, angle_range: u16) -> Result<u16, I2C::Error> {
        self.write_12bits(ConfigRegisters::MAngHi as u8, angle_range)?;
        let verify_value = self.read_12bits(ConfigRegisters::MAngHi as u8)?;
        Ok(verify_value)
    }


    pub fn read_agc(&mut self) -> Result<u16, I2C::Error> {
        let agc = self.read_12bits(StatusRegisters::AGC as u8)?;
        Ok(agc)
    }

    pub fn read_magnitude(&mut self) -> Result<u16, I2C::Error> {
        let magnitude = self.read_12bits(StatusRegisters::MagnitudeHi as u8)?;
        Ok(magnitude)
    }

    pub fn read_angle(&mut self) -> Result<u16, I2C::Error> {
        let angle = self.read_12bits(OutputRegisters::AngleHi as u8)?;
        Ok(angle)
    }

    pub fn read_raw_angle(&mut self) -> Result<u16, I2C::Error> {
        let angle = self.read_12bits(OutputRegisters::RawAngleHi as u8)?;
        Ok(angle)
    }


    pub fn set_temporary_address(&mut self, address: u8) -> Result<(), I2C::Error> {
        self.i2c.write(
            SENSOR_ADDR,
            &[(ConfigRegisters::I2cAddr as u8), address]
        )?;
        self.i2c.write(
            SENSOR_ADDR,
            &[(ConfigRegisters::I2cUpdt as u8), address]
        )?;

        Ok(())
    }

    pub fn read_12bits(&mut self, address: u8) -> Result<u16, I2C::Error> {
        let mut bytes: [u8; 2] = [0x00, 0x00];

        self.i2c.write_read(
            SENSOR_ADDR,
            &[address],
            &mut bytes 
        )?;
        
        Ok(bytes_to_u16(bytes))
    }

    pub fn write_12bits(&mut self, address: u8, value: u16) -> Result<(), I2C::Error> {
        let mut data: [u8; 2];
        data = value.to_be_bytes();

        //Ensure that the only 12bits are used.
        data[0] &= 0x0F;

        self.i2c.write(
            SENSOR_ADDR,
            &[address, data[0], data[1]]
        )?;
        Ok(())
    }

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
    fn i2c_writes_u16() {
        let expectations = [
            I2cTransaction::write(SENSOR_ADDR, vec![0x05, 0x0F, 0xFF]),
            I2cTransaction::write(SENSOR_ADDR, vec![0x06, 0x0A, 0xAA]),
        ];

        let i2c = I2cMock::new(&expectations);
        let mut sensor = AS5600::new(i2c);

        let result = sensor.write_12bits(0x05, 0xFFFF);
        assert!(result.is_ok());

        let result = sensor.write_12bits(0x06, 0xAAAA);
        assert!(result.is_ok());

        sensor.i2c.done();
    }

    #[test]
    fn i2c_read_12bits() {
        let expectations = [
            I2cTransaction::write_read(SENSOR_ADDR, vec![0x05], vec![0x0F, 0xFF]),
            I2cTransaction::write_read(SENSOR_ADDR, vec![0x06], vec![0x00, 0x00]),
        ];

        let i2c = I2cMock::new(&expectations);
        let mut sensor = AS5600::new(i2c);

        let result = sensor.read_12bits(0x05);
        assert_eq!(result.unwrap(), 0x0FFF);

        let result = sensor.read_12bits(0x06);
        assert_eq!(result.unwrap(), 0x0000);

        sensor.i2c.done();
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
        let expect = [
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(ConfigRegisters::MPosHi as u8), 0x0F, 0xFF]
            ),
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(ConfigRegisters::MPosHi as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);

        let mut sensor = AS5600::new(i2c);
      
        let start: u16 = 0x0FFF;
        let result = sensor.config_stop_position(start);
        assert_eq!(result.unwrap(), 0x0FFF);

        sensor.i2c.done();
    }


    #[test]
    fn test_config_angular_range() {
        let expect = [
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(ConfigRegisters::MAngHi as u8), 0x0F, 0xFF]
            ),
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(ConfigRegisters::MAngHi as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);

        let mut sensor = AS5600::new(i2c);
      
        let angle_range: u16 = 0x0FFF;
        let result = sensor.config_angular_range(angle_range);
        assert_eq!(result.unwrap(), 0x0FFF);

        sensor.i2c.done();
    }


    #[test]
    fn test_reading_agc() {
        let expect = [
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(StatusRegisters::AGC as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);
        let mut sensor = AS5600::new(i2c);
        
        let result = sensor.read_agc();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0FFF);

        sensor.i2c.done();
    }

    #[test]
    fn test_reading_magnitude() {
        let expect = [
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(StatusRegisters::MagnitudeHi as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);
        let mut sensor = AS5600::new(i2c);
        
        let result = sensor.read_magnitude();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0FFF);

        sensor.i2c.done();
    }

    #[test]
    fn test_reading_angle() {
        let expect = [
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(OutputRegisters::AngleHi as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);
        let mut sensor = AS5600::new(i2c);
        
        let result = sensor.read_angle();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0FFF);

        sensor.i2c.done();
    }


    #[test]
    fn test_reading_raw_angle() {
        let expect = [
            I2cTransaction::write_read(
                SENSOR_ADDR,
                vec![(OutputRegisters::RawAngleHi as u8)],
                vec![0x0F, 0xFF],
            ),
        ];

        let i2c = I2cMock::new(&expect);
        let mut sensor = AS5600::new(i2c);
        
        let result = sensor.read_raw_angle();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0FFF);

        sensor.i2c.done();
    }

    #[test]
    fn test_tempoary_address_change() {
        let expect = [
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(ConfigRegisters::I2cAddr as u8), 0x41],
            ),
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(ConfigRegisters::I2cUpdt as u8), 0x41],
            ),
        ];

        let i2c = I2cMock::new(&expect);
        let mut sensor = AS5600::new(i2c);
       
        let address: u8 = 0x41;
        let result = sensor.set_temporary_address(address);
        assert!(result.is_ok());

        sensor.i2c.done();
    }
    
}
