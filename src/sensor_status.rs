//!Bits and their meanings Check the datasheet for version 1.12
//!URL: https://ams-osram.com/support/download-center?search=AS5600
//!


const MAGNET_HIGH: u8 = 8;
const MAGNET_LOW: u8 = 16;
const MAGNET_DETECTED: u8 = 32;

pub struct SensorStatus{
    pub status: u8,
}

impl SensorStatus{
    pub fn new(status: u8) -> SensorStatus {
        SensorStatus{ status }
    }
}

#[cfg(test)]
mod test_status {
    use super::*;

    #[test]
    fn self_test() {
        assert!(true);
    }


}
