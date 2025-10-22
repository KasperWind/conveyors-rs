use std::fmt::Display;
use std::sync::Arc;

use spdlog::info;
use spdlog::sink::FileSink;

pub struct Conv {
    pub transfer_in: bool,
    pub transfer_out: bool,
    pub motor: bool,
    pub sensor: bool,
}

#[derive(Debug, Default)]
pub struct State {
    pub started: bool,
    pub motor1: bool,
    pub transfer1: bool,
    pub sensor1: bool,
    pub motor2: bool,
    pub sensor2: bool,
    pub transfer2: bool,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "started: {:05},  ", self.started)?;
        write!(f, "motor1: {:05},  ", self.motor1)?;
        write!(f, "sensor1: {:05},  ", self.sensor1)?;
        write!(f, "motor2: {:05},  ", self.motor2)?;
        write!(f, "sensor2: {:05},  ", self.sensor2)
    }
}

fn update_state(i: usize, state: &mut State) {
    if i == 5 {
        state.started = true;
    }

    if i == 10 {
        state.sensor1 = true;
    }

    if i == 15 {
        state.sensor1 = false;
    }

    if i == 20 {
        state.sensor2 = true;
    }

    if i == 25 {
        state.sensor2 = false;
    }

    state.motor1 = state.started && state.transfer1;
    state.motor2 = state.started && (state.transfer1 || state.transfer2);

    if state.started && state.sensor1 && !state.sensor2 && !state.transfer1 {
        state.transfer1 = true;
    }

    if state.transfer1 && state.started && !state.sensor1 && state.sensor2 {
        state.transfer1 = false;
    }

    if state.started && state.sensor2 && !state.transfer1 && !state.transfer2 {
        state.transfer2 = true;
    }

    if state.transfer2 && state.started && !state.sensor1 && !state.sensor2 {
        state.transfer2 = false;
    }
}

fn main() {
    let path = "log.log";
    let logger = spdlog::default_logger()
        .fork_with(|new| {
            let file_sink = Arc::new(
                FileSink::builder()
                    .path(path)
                    .build()
                    .expect("Log file sink should be created"),
            );
            new.sinks_mut().push(file_sink);
            Ok(())
        })
        .expect("Log could be created");
    spdlog::set_default_logger(logger);
    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);
    info!("Logging initialized.");

    let mut state = State::new();
    info!("Just created state:          {state}");

    for i in 0..50 {
        info!("state at start of loop {i:04}: {state}");
        update_state(i, &mut state);
        info!("state at end of loop   {i:04}: {state}");
    }

    spdlog::default_logger().flush();
}
