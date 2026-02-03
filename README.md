# SwitchClientLib

A Rust library for controlling a Nintendo Switch via a Raspberry Pi Pico W running [PicoSwitchController](https://github.com/Yvonne-Aizawa/PicoSwitchController).

PicoSwitchController is firmware that turns a Pico W into a Bluetooth Nintendo Switch Pro Controller, accepting commands over USB serial. This library provides a typed Rust API for sending those commands.

## How it works

```
Host PC  --USB serial-->  Pico W  --Bluetooth-->  Nintendo Switch
         (this library)           (firmware)
```

The Pico W pairs with the Switch as a Pro Controller. This library opens a serial connection to the Pico and sends newline-terminated ASCII commands to press buttons, move sticks, and set controller state.

## Usage

Add `switchcontroller` as a dependency:

```toml
[dependencies]
switchcontroller = { path = "switchcontroller" }
```

```rust
use switchcontroller::{Button, ControllerState, Stick, SwitchController};

fn main() {
    let mut ctrl = SwitchController::open("/dev/ttyACM0", 115200)
        .expect("failed to open serial port");

    // Press A
    ctrl.press(&[Button::A]).unwrap();

    // Hold ZR, press A, then release
    ctrl.hold(&[Button::ZR]).unwrap();
    ctrl.sleep(0.1).unwrap();
    ctrl.press(&[Button::A]).unwrap();
    ctrl.release(&[Button::ZR]).unwrap();

    // Move left stick right, then center it
    ctrl.stick(Stick::Left, 1.0, 0.0).unwrap();
    ctrl.sleep(1.0).unwrap();
    ctrl.stick(Stick::Left, 0.0, 0.0).unwrap();

    // Set full controller state at once
    let mut state = ControllerState::new();
    state.set_button(Button::A, true);
    state.set_left_stick(0.5, -1.0);
    ctrl.state(&state).unwrap();
}
```

## API

### `SwitchController`

| Method | Description |
|--------|-------------|
| `open(path, baud_rate)` | Open a serial connection to the Pico |
| `from_port(port)` | Wrap an already-opened serial port |
| `press(buttons)` | Press and release buttons (held for one frame) |
| `hold(buttons)` | Hold buttons until released |
| `release(buttons)` | Release held buttons |
| `stick(stick, h, v)` | Set stick position (-1.0 to 1.0) |
| `state(state)` | Set entire controller state in one command |
| `sleep(seconds)` | Pause command processing on the device |

### `Button`

`A`, `B`, `X`, `Y`, `L`, `R`, `ZL`, `ZR`, `Plus`, `Minus`, `Home`, `Capture`, `LStick`, `RStick`, `DpadUp`, `DpadDown`, `DpadLeft`, `DpadRight`

### `Stick`

`Left`, `Right`

### `ControllerState`

Builder for the `STATE` command. Set individual buttons and stick positions, then send with `ctrl.state(&state)`.

## Runner

The workspace includes a `runner` binary for quick testing:

```sh
cargo run -p runner -- /dev/ttyACM0 115200
```

## System dependencies

On Linux, the `serialport` crate requires `libudev-dev`:

```sh
sudo apt install libudev-dev
```
