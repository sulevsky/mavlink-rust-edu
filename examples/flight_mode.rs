use std::{sync::Arc, thread, time::Duration};

use mavlink::{MavConnection, ardupilotmega::MavMessage, error::MessageReadError};

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";
const MAV_MODE_FLAG_CUSTOM_MODE_ENABLED: f32 = 1f32;

fn main() {
    println!("Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();
    println!("Connected to {}", SERVER_ADDRESS);
    let (autopilot_system_id, autopilot_component_id) = fetch_system_id(&connection);
    println!("Vehicle > autopilot_system_id: {autopilot_system_id}");
    println!("Vehicle > autopilot_component_id: {autopilot_component_id}");
    let arc = Arc::new(connection);
    listen_for_flight_mode(arc.clone());

    thread::sleep(Duration::from_secs(5));
    let header = mavlink::MavHeader::default();
    let guided_command = mavlink::ardupilotmega::COMMAND_LONG_DATA {
        target_system: autopilot_system_id,
        target_component: autopilot_component_id,
        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_DO_SET_MODE,
        param1: MAV_MODE_FLAG_CUSTOM_MODE_ENABLED,
        param2: FlightMode::GUIDED.to_f32(),
        ..Default::default()
    };
    let stabilize_command = mavlink::ardupilotmega::COMMAND_LONG_DATA {
        param2: FlightMode::STABILIZE.to_f32(),
        ..guided_command.clone()
    };
    let set_flight_mode_guided_message =
        mavlink::ardupilotmega::MavMessage::COMMAND_LONG(guided_command);
    println!(
        "Sending set flight mode GUIDED command: {:?}",
        set_flight_mode_guided_message,
    );
    arc.send(&header, &set_flight_mode_guided_message).unwrap();
    thread::sleep(Duration::from_secs(5));
    let set_flight_mode_stabilize_message =
        mavlink::ardupilotmega::MavMessage::COMMAND_LONG(stabilize_command);
    println!(
        "Sending set flight mode STABILIZE command: {:?}",
        set_flight_mode_stabilize_message,
    );
    arc.send(&header, &set_flight_mode_stabilize_message)
        .unwrap();
    thread::sleep(Duration::from_secs(500));
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum FlightMode {
    STABILIZE = 0,
    ACRO = 1,
    ALT_HOLD = 2,
    AUTO = 3,
    GUIDED = 4,
    LOITER = 5,
    RTL = 6,
    UNKNOWN = isize::max_value(),
}

impl FlightMode {
    fn from(value: u32) -> Self {
        match value {
            0 => FlightMode::STABILIZE,
            1 => FlightMode::ACRO,
            2 => FlightMode::ALT_HOLD,
            3 => FlightMode::AUTO,
            4 => FlightMode::GUIDED,
            5 => FlightMode::LOITER,
            6 => FlightMode::RTL,
            _ => FlightMode::UNKNOWN,
        }
    }
    fn to_f32(self) -> f32 {
        (self as isize) as f32
    }
}
fn fetch_system_id(connection: &Box<dyn MavConnection<MavMessage> + Send + Sync>) -> (u8, u8) {
    loop {
        match connection.try_recv() {
            Ok((header, _)) => {
                return (header.system_id, header.component_id);
            }
            Err(e) => {
                println!("recv error: {e:?}");
                panic!()
            }
        }
    }
}

fn listen_for_flight_mode(connection: Arc<Box<dyn MavConnection<MavMessage> + Send + Sync>>) {
    thread::spawn({
        move || loop {
            match connection.try_recv() {
                Ok((_, msg)) => match msg {
                    mavlink::ardupilotmega::MavMessage::HEARTBEAT(data) => {
                        let custom_mode = data.custom_mode;
                        let mode = FlightMode::from(custom_mode);
                        println!("Vehicle > flight mode: {:?}({})", mode, custom_mode);
                    }
                    mavlink::ardupilotmega::MavMessage::COMMAND_ACK(data) => {
                        println!("Vehicle > Command {:?} is {:?}", data.command, data.result);
                    }
                    _ => {
                        // ignore other messages
                    }
                },
                Err(MessageReadError::Io(e)) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    } else {
                        println!("recv error: {e:?}");
                        panic!()
                    }
                }
                Err(e) => {
                    println!("recv error: {e:?}");
                    panic!()
                }
            }
        }
    });
}
