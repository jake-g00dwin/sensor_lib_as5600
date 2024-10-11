//! AS5600 I2C driver:
//!
//! Provides a unit tested driver to access the AS5600 series of sensor modules, 
//! focuses on:
//! 
//! - Providing reliable data.
//! - A safer interface to an i2c sensor.
//! - No infinite loops.
//! - No assumption of reliable hardware(passes back error messages) 
//!
//!
//! To see a full functional demo of the crate/repo in use you can checkout the
//! repo at the following URL:
//! [`ch32v203_as5600_demo`](https://github.com/jake-g00dwin/ch32v203_as5600_demo)
//!
//!```rust,ignore
//!
//! use sensor_lib_as5600::AS5600;
//!
//! /*--SNIP--*/
//!
//!#[entry]
//!fn main() -> ! {
//! 
//! /*--SNIP--*/
//!
//! //Device specific I2C pins(ch32v203kxt6)
//! let scl = p.PB6;
//! let sda = p.PB7;
//!
//! let i2c_config = hal::Config::default();
//! let i2c = I2c::new_blocking(p.I2C1, scl, sda, Hertz::hz(100_000), Default::default());
//!
//! let addr = 0x36;
//! let mut as5600 = AS5600::new(i2c, addr);
//!
//!
//! let mut angle: u16 = 0;
//!
//! angle = as5600.read_angle().unwrap();
//!
//!}
//!```
//!
//!The above example is shortened code demoing how you can use the sensor
//!library.



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
    address: u8,
}

impl <I2C: I2c> AS5600<I2C> {

    /// Creates a new instance of the AS5600 structure.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address}
    }

    /// Reads the status of the AS5600 sensor.
    pub fn read_status(&mut self) -> Result<SensorStatus, I2C::Error> {
        let mut buf: [u8; 1] = [0x00];
        
        self.i2c.write_read(
            self.address,
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

    /// Reads the current AGC(automatic gain control).
    ///
    /// AGC compensates for variations in magnetic field strength. The AGC
    /// register indicates gain. In 5v mode the range is 0-255 while in 3.3v
    /// mode the range is 0-128.
    ///
    /// You should ensure that it's centered in range when installed for your
    /// applicaiton.
    pub fn read_agc(&mut self) -> Result<u16, I2C::Error> {
        let agc = self.read_12bits(StatusRegisters::AGC as u8)?;
        Ok(agc)
    }

    pub fn read_magnitude(&mut self) -> Result<u16, I2C::Error> {
        let magnitude = self.read_12bits(StatusRegisters::MagnitudeHi as u8)?;
        Ok(magnitude)
    }

    /// Reads the angle value from the output registers.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut as5600 = AS5600::new(i2c, addr);
    ///
    /// //Reads the 12bits of angle info giving a value between 0-4096
    /// let angle: u16 = as5600.read_angle()?;
    ///
    /// ```
    pub fn read_angle(&mut self) -> Result<u16, I2C::Error> {
        let angle = self.read_12bits(OutputRegisters::AngleHi as u8)?;
        Ok(angle)
    }

    /// Reads the unscaled and unmodified angle.
    ///
    /// The scaled output is availble through the `read_angle()` funciton.
    pub fn read_raw_angle(&mut self) -> Result<u16, I2C::Error> {
        let angle = self.read_12bits(OutputRegisters::RawAngleHi as u8)?;
        Ok(angle)
    }

    /// Sets a temporary i2c address for the device.
    pub fn set_temporary_address(&mut self, address: u8) -> Result<(), I2C::Error> {
        self.i2c.write(
            self.address,
            &[(ConfigRegisters::I2cAddr as u8), address]
        )?;
        self.i2c.write(
            self.address,
            &[(ConfigRegisters::I2cUpdt as u8), address]
        )?;

        Ok(())
    }

    /// Burn the current configuration settings; permanant action!
    pub fn burn_setting(&mut self) -> Result<(), I2C::Error> {
        self.i2c.write(
            self.address,
            &[(BurnCommands::Burn as u8), 0x40]
        )?;
        Ok(())
    }

    /// Reads 12bits of information from i2c device from a address(u8).
    ///
    /// # Examples
    ///
    /// ```
    /// //Reads the Hi+Lo angle registers into a 16bit u16 return value.
    /// let angle = self.read_12bits(OutputRegisters::AngleHi as u8)?;
    ///
    /// ```
    pub fn read_12bits(&mut self, address: u8) -> Result<u16, I2C::Error> {
        let mut bytes: [u8; 2] = [0x00, 0x00];

        self.i2c.write_read(
            self.address,
            &[address],
            &mut bytes 
        )?;
        
        Ok(bytes_to_u16(bytes))
    }


    /// Writes 12bits of information to the i2c device to a address(u8).
    ///
    /// # Examples
    ///
    /// ```
    /// //Writes 12bits of info into the configuration register
    /// // where 'start' is a u16 value.
    /// self.write_12bits(ConfigRegisters::ZPosHi as u8, start)?;
    ///
    /// ```
    pub fn write_12bits(&mut self, address: u8, value: u16) -> Result<(), I2C::Error> {
        let mut data: [u8; 2];
        data = value.to_be_bytes();

        //Ensure that the only 12bits are used.
        data[0] &= 0x0F;

        self.i2c.write(
            self.address,
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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);

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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);

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
       
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
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

        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
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

        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
      
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

        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
      
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

        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
      
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

        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
      
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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
        
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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
        
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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
        
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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
        
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
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
       
        let address: u8 = 0x41;
        let result = sensor.set_temporary_address(address);
        assert!(result.is_ok());

        sensor.i2c.done();
    }


    #[test]
    fn test_burn_setting_cmd() {
        let expect = [
            I2cTransaction::write(
                SENSOR_ADDR,
                vec![(BurnCommands::Burn as u8), 0x40],
            ),
        ];

        let i2c = I2cMock::new(&expect);
        let mut sensor = AS5600::new(i2c, SENSOR_ADDR);
       
        let result = sensor.burn_setting();
        assert!(result.is_ok());

        sensor.i2c.done();
    }

}
