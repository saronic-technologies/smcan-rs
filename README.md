# smcan-rs

A `no_std` rust crate to generate and parse CAN Frames for the [Salient Motion
SM-CAN Protocol](https://docs.salientmotion.com/software-tool/can-communication).

## Install the crate

``` sh
cargo add smcan
```

## Examples

`socketcan_recrev_cmd.rs` will send and receive CAN-FD Frames via the linux
socketcan interface using
[`socketcan-rs`](https://github.com/socketcan-rs/socketcan-rs)

To run the example first bring up a `vcan0` interface:

``` sh
sudo modprobe vcan
sudo ip link add vcan0 type vcan
sudo ip link set vcan0 up
```

Then run `socketcan_recrev_cmd.rs`:

``` sh
cargo run --example socketcan_recrev_cmd
```
