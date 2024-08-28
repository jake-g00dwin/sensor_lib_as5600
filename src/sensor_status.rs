//!Bits and their meanings Check the datasheet for version 1.12
//!URL: https://ams-osram.com/support/download-center?search=AS5600
//!


const MAGNET_HIGH: u8 = 8;
const MAGNET_LOW: u8 = 16;
const MAGNET_DETECTED: u8 = 32;

const MAG_HIGH_BM: u8 = 1<<3;
const MAG_LOW_BM: u8 = 1<<4;
const MAG_DETECTED_BM: u8 = 1<<5;

#[derive(Debug)]
pub struct SensorStatus{
    pub status: u8,
}

impl SensorStatus{
    pub fn new(status: u8) -> SensorStatus {
        SensorStatus{ status }
    }

    pub fn is_magnet_high(&self) -> bool {
        (self.status & MAG_HIGH_BM) == MAGNET_HIGH
    }

    pub fn is_magnet_low(&self) -> bool {
        (self.status & MAG_LOW_BM) == MAGNET_LOW
    }

    pub fn is_magnet_detected(&self) -> bool {
        (self.status & MAG_DETECTED_BM) == MAGNET_DETECTED 
    }

}

#[cfg(test)]
mod test_status {
    use super::*;

    #[test]
    fn self_test() {
        assert!(true);
    }

    #[test]
    fn new_status() {
        let s = SensorStatus::new(0x08);

        assert_eq!(s.status, 0x08);
    }

    #[test]
    fn magnet_high_status() {
        let s = SensorStatus::new(MAGNET_HIGH);
        assert_eq!(s.is_magnet_high(), true);
        assert_eq!(s.is_magnet_low(), false);
        assert_eq!(s.is_magnet_detected(), false);
    }

    #[test]
    fn magnet_low_status() {
        let s = SensorStatus::new(MAGNET_LOW);
        assert_eq!(s.is_magnet_low(), true);
        assert_eq!(s.is_magnet_high(), false);
        assert_eq!(s.is_magnet_detected(), false);
    }

    #[test]
    fn magnet_detected_status() {
        let s = SensorStatus::new(MAGNET_DETECTED);
        assert_eq!(s.is_magnet_detected(), true);
    }
}
