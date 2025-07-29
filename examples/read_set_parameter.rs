use std::{thread, time::Duration};

use mavlink::{
    MavConnection,
    ardupilotmega::{MavMessage, PARAM_VALUE_DATA},
    error::MessageReadError,
};

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";

fn main() {
    println!("Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();
    println!("Connected to {}", SERVER_ADDRESS);
    let (autopilot_system_id, autopilot_component_id) = fetch_system_id(&connection);
    println!("autopilot_system_id: {autopilot_system_id}");
    println!("autopilot_component_id: {autopilot_component_id}");
    let header = mavlink::MavHeader::default();
    let param_request_read_message = mavlink::ardupilotmega::MavMessage::PARAM_REQUEST_READ(
        mavlink::ardupilotmega::PARAM_REQUEST_READ_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
            param_id: encode_param_id("SIM_SPEEDUP"),
            param_index: -1,
        },
    );
    println!(
        "Sending param request read message: {:?}",
        param_request_read_message
    );
    connection
        .send(&header, &param_request_read_message)
        .unwrap();
    listen_for_param_read_messages(&connection);

    let param_request_set_message =
        mavlink::ardupilotmega::MavMessage::PARAM_SET(mavlink::ardupilotmega::PARAM_SET_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
            param_id: encode_param_id("SIM_SPEEDUP"),
            param_value: 42.0,
            param_type: mavlink::ardupilotmega::MavParamType::MAV_PARAM_TYPE_REAL32,
        });
    println!(
        "Sending param request set message: {:?}",
        param_request_set_message
    );
    connection
        .send(&header, &param_request_set_message)
        .unwrap();

    println!("Reading updated parameter");
    listen_for_param_read_messages(&connection);
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

fn listen_for_param_read_messages(connection: &Box<dyn MavConnection<MavMessage> + Send + Sync>) {
    loop {
        match connection.try_recv() {
            Ok((_, msg)) => match msg {
                mavlink::ardupilotmega::MavMessage::PARAM_VALUE(data) => {
                    print_param_data(&data);
                    return;
                }
                _ => {
                    // ignore other messages
                }
            },
            Err(MessageReadError::Io(e)) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    println!("No new messages");
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
}
fn print_param_data(data: &PARAM_VALUE_DATA) {
    let param_id = decode_param_id(&data.param_id);
    println!("{}", data.param_index);
    println!("id:    {}", param_id);
    println!("value: {}", data.param_value);
    println!();
}

fn decode_param_id(param_id: &[u8; 16]) -> String {
    param_id
        .iter()
        .filter(|&b| *b != 0)
        .map(|&b| char::from(b))
        .collect()
}

fn encode_param_id(param_id: &str) -> [u8; 16] {
    let mut result = [0u8; 16];
    let bytes = param_id.as_bytes();
    let len = bytes.len().min(16);
    result[..len].copy_from_slice(&bytes[..len]);
    result
}
