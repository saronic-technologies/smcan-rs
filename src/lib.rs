#![no_std]
use embedded_can::{Id, Frame, ExtendedId};

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

pub enum Message {
    Telemetry(Telemetry),
    MotorCmd(MotorCmd),
    Unsupported,
}

// impl<T: Frame> Into<T> for Message {
//     fn into(self) -> T {
//         match self {
//             Telemetry(t) => { },
//             MotorCmd(m) => {  },
//             Unsupported => panic!("oops"),
//         }
//     }
// }

impl<T: Frame> From<T> for Message {
    fn from(frame: T) -> Self {
        // Frame should be a CAN-FD frame
        let id = match frame.id() {
            Id::Standard(_) => return Self::Unsupported,
            Id::Extended(eid) => eid.as_raw(),
        };

        match id {
            // ctrl_id
            0x80ec0191 => {
                let data: &[u8] = frame.data();
                Self::MotorCmd(MotorCmd{
                    cmd_value: u16::from_be_bytes([data[0], data[1]])
                })
            },
            //telem_id
            0x9feeab01 => {
                let data: &[u8] = frame.data();
                Self::Telemetry(Telemetry {
                    motor_speed: f32::from_ne_bytes(data[0..3].try_into().unwrap()),
                    motor_current: f32::from_ne_bytes(data[4..7].try_into().unwrap()),
                    battery_voltage: f32::from_ne_bytes(data[8..11].try_into().unwrap()),
                    battery_current: f32::from_ne_bytes(data[12..15].try_into().unwrap()),
                    commanded_value: f32::from_ne_bytes(data[16..19].try_into().unwrap()),
                    mosfet_temp: f32::from_ne_bytes(data[20..23].try_into().unwrap()),
                })
            },
            _ => {
                Self::Unsupported
            }
        }


    }
}
