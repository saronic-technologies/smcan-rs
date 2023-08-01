
#![no_std]
use embedded_can::{Frame, StandardId};

pub struct Telemetry {
    motor_speed: f32,
    motor_current: f32,
    battery_voltage: f32,
    battery_current: f32,
    commanded_value: f32,
    mosfet_temp: f32,
}

pub struct MotorCmd {
    cmd_value: u16,
}

impl<T: Frame> From<T> for Telemetry {
    fn from(frame: T) -> Self {
        // Frame should be a CAN-FD frame
        let data: &[u8] = frame.data();
        Self {
            motor_speed: f32::from_ne_bytes(data[0..3].try_into().unwrap()),
            motor_current: f32::from_ne_bytes(data[4..7].try_into().unwrap()),
            battery_voltage: f32::from_ne_bytes(data[8..11].try_into().unwrap()),
            battery_current: f32::from_ne_bytes(data[12..15].try_into().unwrap()),
            commanded_value: f32::from_ne_bytes(data[16..19].try_into().unwrap()),
            mosfet_temp: f32::from_ne_bytes(data[20..23].try_into().unwrap()),
        }
    }
}
