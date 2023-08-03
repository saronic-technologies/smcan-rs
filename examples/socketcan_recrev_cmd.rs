use anyhow::Context;
use embedded_can::{blocking::Can, Frame as EmbeddedFrame};
use smcan::{Message, MotorCmd, Telemetry};
use socketcan::{CanAnyFrame, CanFdFrame, CanFdSocket, Socket};
use std::env;

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "vcan0".into());

    let mut read_sock = CanFdSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;

    let mut write_sock = CanFdSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;

    std::thread::spawn(move || {
        loop {
            let frame = read_sock.read_frame().context("Receiving Frame").unwrap();
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
    });

    let sleep_time = std::time::Duration::from_millis(500);

    loop {
        let cmd = MotorCmd::new(100);
        let frame = Message::MotorCmd(cmd).framify::<CanFdFrame>().unwrap();
        write_sock.write_frame(&frame).context("Transmitting frame")?;

        std::thread::sleep(sleep_time);

        let telem = Telemetry::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let tfd = Message::Telemetry(telem).framify::<CanFdFrame>().unwrap();
        write_sock.write_frame(&tfd).context("Transmitting frame")?;

        std::thread::sleep(sleep_time);
    }
}
