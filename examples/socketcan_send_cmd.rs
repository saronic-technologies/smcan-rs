use anyhow::Context;
use embedded_can::{blocking::Can, Frame as EmbeddedFrame};
use smcan::{Message, MotorCmd, Telemetry};
use socketcan::{CanAnyFrame, CanFdFrame, CanFdSocket, Socket};
use std::env;

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "vcan0".into());

    let mut sock = CanFdSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;

    let sleep_time = std::time::Duration::from_millis(500);

    loop {
        let cmd = MotorCmd::new(100);
        let frame = Message::MotorCmd(cmd).framify::<CanFdFrame>().unwrap();
        sock.write_frame(&frame).context("Transmitting frame")?;

        std::thread::sleep(sleep_time);

        let telem = Telemetry::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let tfd = Message::Telemetry(telem).framify::<CanFdFrame>().unwrap();
        sock.write_frame(&tfd).context("Transmitting frame")?;

        std::thread::sleep(sleep_time);
    }
}
