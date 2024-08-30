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

#[repr(u16)]
#[allow(dead_code)]
pub enum PowerModeBM{
    Nom = 0x0000,
    LPM1 = 0x0001,
    LPM2 = 0x0002,
    LPM3 = 0x0003,
}


#[repr(u16)]
#[allow(dead_code)]
pub enum HysteresisBM{
    HysteresisOff = 0x0000,
    HysteresisLSB1 = 0x0000 | (1<<2),
    HysteresisLSB2 = 0x0000 | (1<<3),
    HysteresisLSB3 = 0x0000 | (1<<2)|(1<<3),
}

#[repr(u16)]
#[allow(dead_code)]
pub enum OutputStageBM{
    VDD = 0,
    VDD1 = (1<<4),
    PWM = (1<<5),
}


#[repr(u16)]
#[allow(dead_code)]
pub enum PWMFrequencyBM{
    F115hz = 0,
    F230hz = (1<<6),
    F460hz = (1<<7),
    F920hz = (1<<7)|(1<<6),
}


#[repr(u16)]
#[allow(dead_code)]
pub enum SlowFilterBM{
    Fltr16x = 0,
    Fltr8x = (1<<8),
    Fltr4x = (1<<9),
    Fltr2x = (1<<9)|(1<<8),
}

#[repr(u16)]
#[allow(dead_code)]
pub enum FastFilterThresholdBM{
    SlowOnly = 0,
    LSB6 = (1<<10),
    LSB7 = (1<<11),
    LSB9 = (1<<11)|(1<<10),
    LSB18 = (1<<12),
    LSB21 = (1<<12)|(1<<10),
    LSB24 = (1<<12)|(1<<11),
    LSB10 = (1<<12)|(1<<11)|(1<<10),
}

#[repr(u16)]
#[allow(dead_code)]
pub enum WatchDogBM{
    Off = 0,
    On = (1<<13),
}


