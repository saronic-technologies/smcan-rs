#![no_std]
use embedded_can::{ExtendedId, Frame, Id};

#[derive(Debug)]
pub struct Telemetry {
    pub motor_speed: f32,
    pub motor_current: f32,
    pub battery_voltage: f32,
    pub battery_current: f32,
    pub commanded_value: f32,
    pub mosfet_temp: f32,
}

#[derive(Debug)]
pub struct MotorCmd {
    pub cmd_value: u16,
}

pub enum Message {
    Telemetry(Telemetry),
    MotorCmd(MotorCmd),
    Unsupported,
}

impl MotorCmd {
    pub fn new(cmd_value: u16) -> Self {
        Self { cmd_value }
    }
}

impl Telemetry {
    pub fn new(
        motor_speed: f32,
        motor_current: f32,
        battery_voltage: f32,
        battery_current: f32,
        commanded_value: f32,
        mosfet_temp: f32,
    ) -> Self {
        Self {
            motor_speed,
            motor_current,
            battery_voltage,
            battery_current,
            commanded_value,
            mosfet_temp,
        }
    }
}

impl Message {
    pub fn framify<T: Frame>(&self) -> Option<T> {
        match self {
            Self::Telemetry(t) => {
                let id = ExtendedId::new(0x1feeab01).unwrap();
                let mut b = [0u8; 24];
                b[0..4].copy_from_slice(&t.motor_speed.to_be_bytes());
                b[4..8].copy_from_slice(&t.motor_current.to_be_bytes());
                b[8..12].copy_from_slice(&t.battery_voltage.to_be_bytes());
                b[12..16].copy_from_slice(&t.battery_current.to_be_bytes());
                b[16..20].copy_from_slice(&t.commanded_value.to_be_bytes());
                b[20..24].copy_from_slice(&t.mosfet_temp.to_be_bytes());
                T::new(id, &b)
            }
            Self::MotorCmd(m) => {
                let id = ExtendedId::new(0x00ec0191).unwrap();
                T::new(id, &m.cmd_value.to_be_bytes())
            }
            Self::Unsupported => return None,
        }
    }
}

impl<T: Frame> From<T> for Message {
    fn from(frame: T) -> Self {
        // Frame should be a CAN-FD frame
        let id = match frame.id() {
            Id::Standard(_) => return Self::Unsupported,
            Id::Extended(eid) => eid.as_raw(),
        };

        match id {
            // ctrl_id
            0x00ec0191 => {
                let data: &[u8] = frame.data();
                Self::MotorCmd(MotorCmd {
                    cmd_value: u16::from_be_bytes([data[0], data[1]]),
                })
            }
            //telem_id
            0x1feeab01 => {
                let data: &[u8] = frame.data();
                Self::Telemetry(Telemetry {
                    motor_speed: f32::from_be_bytes(data[0..4].try_into().unwrap()),
                    motor_current: f32::from_be_bytes(data[4..8].try_into().unwrap()),
                    battery_voltage: f32::from_be_bytes(data[8..12].try_into().unwrap()),
                    battery_current: f32::from_be_bytes(data[12..16].try_into().unwrap()),
                    commanded_value: f32::from_be_bytes(data[16..20].try_into().unwrap()),
                    mosfet_temp: f32::from_be_bytes(data[20..24].try_into().unwrap()),
                })
            }
            _ => Self::Unsupported,
        }
    }
}
