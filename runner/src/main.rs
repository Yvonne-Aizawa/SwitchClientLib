use switchcontroller::{Button, ControllerState, Stick, SwitchController};

fn main() {
    let port = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: runner <serial-port> [baud-rate]");
        eprintln!("  e.g. runner /dev/ttyACM0 115200");
        std::process::exit(1);
    });
    let baud: u32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(115200);

    let mut ctrl = SwitchController::open(&port, baud).expect("failed to open serial port");

    // Press A
    ctrl.press(&[Button::A]).unwrap();
    ctrl.sleep(1.0).unwrap();
    ctrl.press(&[Button::Y]).unwrap();


    // // Hold ZR, press A, then release ZR
    // ctrl.hold(&[Button::ZR]).unwrap();
    // ctrl.sleep(0.1).unwrap();
    // ctrl.press(&[Button::A]).unwrap();
    // ctrl.release(&[Button::ZR]).unwrap();

    // // Move left stick right
    // ctrl.stick(Stick::Left, 1.0, 0.0).unwrap();
    // ctrl.sleep(1.0).unwrap();
    ctrl.stick(Stick::Left, 0.0, 0.0).unwrap();
    ctrl.stick(Stick::Right, 0.0, 0.0).unwrap();

    // // Set full state
    // let mut state = ControllerState::new();
    // state.set_button(Button::A, true);
    // state.set_left_stick(0.5, -1.0);
    // ctrl.state(&state).unwrap();

    println!("Done.");
}
