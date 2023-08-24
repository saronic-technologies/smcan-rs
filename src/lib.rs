#![no_std]
use embedded_can::{ExtendedId, Frame, Id};

use binrw::{binrw, BinRead, BinWrite};
use binrw::io::Cursor;

#[binrw]
#[brw(big)]
#[derive(Debug)]
pub struct Telemetry {
    pub motor_speed: f32,
    pub motor_current: f32,
    pub battery_voltage: f32,
    pub battery_current: f32,
    pub commanded_value: f32,
    pub mosfet_temp: f32,
}

#[binrw]
#[brw(big)]
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
                let mut b = Cursor::new([0u8; 24]);
                let _ = t.write_be(&mut b);
                let bytes = b.into_inner();
                T::new(id, &bytes)
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
                let mut bytes = Cursor::new(data);
                Self::MotorCmd(MotorCmd::read_be(&mut bytes).unwrap())
            }
            //telem_id
            0x1feeab01 => {
                let data: &[u8] = frame.data();
                let mut bytes = Cursor::new(data);
                Self::Telemetry(Telemetry::read_be(&mut bytes).unwrap())
            }
            _ => Self::Unsupported,
        }
    }
}
