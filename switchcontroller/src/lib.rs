use std::fmt;
use std::io::{self, Write};
use std::time::Duration;

/// A Nintendo Switch controller button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    A,
    B,
    X,
    Y,
    L,
    R,
    ZL,
    ZR,
    Plus,
    Minus,
    Home,
    Capture,
    LStick,
    RStick,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
}

impl Button {
    /// All buttons in STATE bit-order (index 0..17).
    pub const ALL: [Button; 18] = [
        Button::A,
        Button::B,
        Button::X,
        Button::Y,
        Button::L,
        Button::R,
        Button::ZL,
        Button::ZR,
        Button::Plus,
        Button::Minus,
        Button::Home,
        Button::Capture,
        Button::LStick,
        Button::RStick,
        Button::DpadUp,
        Button::DpadDown,
        Button::DpadLeft,
        Button::DpadRight,
    ];

    fn as_str(self) -> &'static str {
        match self {
            Button::A => "a",
            Button::B => "b",
            Button::X => "x",
            Button::Y => "y",
            Button::L => "l",
            Button::R => "r",
            Button::ZL => "zl",
            Button::ZR => "zr",
            Button::Plus => "plus",
            Button::Minus => "minus",
            Button::Home => "home",
            Button::Capture => "capture",
            Button::LStick => "l_stick",
            Button::RStick => "r_stick",
            Button::DpadUp => "dpad_up",
            Button::DpadDown => "dpad_down",
            Button::DpadLeft => "dpad_left",
            Button::DpadRight => "dpad_right",
        }
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An analog stick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stick {
    Left,
    Right,
}

impl Stick {
    fn as_str(self) -> &'static str {
        match self {
            Stick::Left => "l_stick",
            Stick::Right => "r_stick",
        }
    }
}

impl fmt::Display for Stick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Full controller state for the `STATE` command.
#[derive(Debug, Clone, Default)]
pub struct ControllerState {
    /// Button state as a bitmask in the order defined by [`Button::ALL`].
    buttons: [bool; 18],
    /// Optional left stick position (horizontal, vertical), each in [-1.0, 1.0].
    pub left_stick: Option<(f32, f32)>,
    /// Optional right stick position (horizontal, vertical), each in [-1.0, 1.0].
    pub right_stick: Option<(f32, f32)>,
}

impl ControllerState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a button's pressed state.
    pub fn set_button(&mut self, button: Button, pressed: bool) -> &mut Self {
        let idx = Button::ALL.iter().position(|&b| b == button).unwrap();
        self.buttons[idx] = pressed;
        self
    }

    /// Set the left stick position.
    pub fn set_left_stick(&mut self, horizontal: f32, vertical: f32) -> &mut Self {
        self.left_stick = Some((horizontal, vertical));
        self
    }

    /// Set the right stick position.
    pub fn set_right_stick(&mut self, horizontal: f32, vertical: f32) -> &mut Self {
        self.right_stick = Some((horizontal, vertical));
        self
    }

    fn to_command(&self) -> String {
        let bits: String = self.buttons.iter().map(|&b| if b { '1' } else { '0' }).collect();
        let mut cmd = format!("STATE {bits}");
        if let Some((lh, lv)) = self.left_stick {
            cmd.push_str(&format!(" {lh} {lv}"));
            if let Some((rh, rv)) = self.right_stick {
                cmd.push_str(&format!(" {rh} {rv}"));
            }
        } else if let Some((rh, rv)) = self.right_stick {
            // Must provide left stick values to include right stick.
            cmd.push_str(&format!(" 0.0 0.0 {rh} {rv}"));
        }
        cmd
    }
}

/// A connection to a Switch controller Pico device over serial.
pub struct SwitchController {
    port: Box<dyn serialport::SerialPort>,
}

impl SwitchController {
    /// Open a serial connection to the Pico at the given path (e.g. `/dev/ttyACM0`).
    pub fn open(path: &str, baud_rate: u32) -> Result<Self, serialport::Error> {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_secs(1))
            .open()?;
        Ok(Self { port })
    }

    /// Create a `SwitchController` from an already-opened serial port.
    pub fn from_port(port: Box<dyn serialport::SerialPort>) -> Self {
        Self { port }
    }

    /// Send a raw newline-terminated command string.
    fn send(&mut self, cmd: &str) -> io::Result<()> {
        write!(self.port, "{cmd}\n")?;
        self.port.flush()
    }

    /// Press and immediately release one or more buttons.
    pub fn press(&mut self, buttons: &[Button]) -> io::Result<()> {
        let names: Vec<&str> = buttons.iter().map(|b| b.as_str()).collect();
        self.send(&format!("PRESS {}", names.join(" ")))
    }

    /// Hold one or more buttons down until explicitly released.
    pub fn hold(&mut self, buttons: &[Button]) -> io::Result<()> {
        let names: Vec<&str> = buttons.iter().map(|b| b.as_str()).collect();
        self.send(&format!("HOLD {}", names.join(" ")))
    }

    /// Release one or more currently held buttons.
    pub fn release(&mut self, buttons: &[Button]) -> io::Result<()> {
        let names: Vec<&str> = buttons.iter().map(|b| b.as_str()).collect();
        self.send(&format!("RELEASE {}", names.join(" ")))
    }

    /// Set an analog stick position. Values range from -1.0 to 1.0.
    pub fn stick(&mut self, stick: Stick, horizontal: f32, vertical: f32) -> io::Result<()> {
        self.send(&format!("STICK {stick} {horizontal} {vertical}"))
    }

    /// Set the entire controller state in a single command.
    pub fn state(&mut self, state: &ControllerState) -> io::Result<()> {
        self.send(&state.to_command())
    }

    /// Pause command processing on the device for the given duration.
    pub fn sleep(&mut self, seconds: f32) -> io::Result<()> {
        self.send(&format!("SLEEP {seconds}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_display() {
        assert_eq!(Button::A.to_string(), "a");
        assert_eq!(Button::ZL.to_string(), "zl");
        assert_eq!(Button::DpadUp.to_string(), "dpad_up");
    }

    #[test]
    fn state_command_no_sticks() {
        let state = ControllerState::new();
        assert_eq!(state.to_command(), "STATE 000000000000000000");
    }

    #[test]
    fn state_command_with_buttons() {
        let mut state = ControllerState::new();
        state.set_button(Button::A, true).set_button(Button::X, true).set_button(Button::Y, true);
        assert_eq!(state.to_command(), "STATE 101100000000000000");
    }

    #[test]
    fn state_command_with_left_stick() {
        let mut state = ControllerState::new();
        state.set_button(Button::A, true);
        state.set_left_stick(0.5, -1.0);
        assert_eq!(state.to_command(), "STATE 100000000000000000 0.5 -1");
    }

    #[test]
    fn state_command_with_both_sticks() {
        let mut state = ControllerState::new();
        state.set_button(Button::A, true);
        state.set_left_stick(0.0, 0.0);
        state.set_right_stick(-1.0, 0.0);
        assert_eq!(state.to_command(), "STATE 100000000000000000 0 0 -1 0");
    }

    #[test]
    fn state_command_with_right_stick_only() {
        let mut state = ControllerState::new();
        state.set_right_stick(-1.0, 0.0);
        assert_eq!(state.to_command(), "STATE 000000000000000000 0.0 0.0 -1 0");
    }
}
