//! This file contains the enums representing the availble I2C registers and
//! commands for the AS5600 IC.

#[repr(u8)]
#[allow(dead_code)]
pub enum ConfigRegisters{
    Zmco = 0x00,
    ZPosHi = 0x01,
    ZPosLo = 0x02,
    MPosHi = 0x03,
    MPosLo = 0x04,
    MAngHi = 0x05,
    MAngLo = 0x06,
    ConfHi = 0x07,
    ConfLo = 0x08,
    I2cAddr = 0x20,
    I2cUpdt = 0x21,
}

#[repr(u8)]
#[allow(dead_code)]
pub enum OutputRegisters{
    RawAngleHi = 0x0C,
    RawAngleLo = 0x0D,
    AngleHi = 0x0E,
    AngleLo = 0x0F,
}


#[repr(u8)]
#[allow(dead_code)]
pub enum StatusRegisters{
    Status = 0x0B,
    AGC = 0x1A,
    MagnitudeHi = 0x1B,
    MagnitudeLo = 0x1C,
}


#[repr(u8)]
#[allow(dead_code)]
pub enum BurnCommands{
    Burn = 0xFF,
}


