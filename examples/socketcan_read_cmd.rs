use anyhow::Context;
use embedded_can::blocking::Can;
use smcan::Message;
use socketcan::{CanAnyFrame, CanFdSocket, Frame, Socket};
use std::env;

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "vcan0".into());

    let mut sock = CanFdSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;

    loop {
        let frame = sock.read_frame().context("Receiving Frame")?;
        let fd_frame = match frame {
            CanAnyFrame::Fd(f) => f,
            _ => panic!("not an FD!"),
        };
        let msg = Message::from(fd_frame);
        match msg {
            Message::MotorCmd(m) => println!("{:?}", m),
            Message::Telemetry(t) => println!("{:?}", t),
            Message::Unsupported => println!("Unsupported CAN Frame"),
        }
    }
}
