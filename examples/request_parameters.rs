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
    let param_request_list_message = mavlink::ardupilotmega::MavMessage::PARAM_REQUEST_LIST(
        mavlink::ardupilotmega::PARAM_REQUEST_LIST_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
        },
    );
    println!(
        "Sending param request list message: {:?}",
        param_request_list_message
    );
    connection
        .send(&header, &param_request_list_message)
        .unwrap();
    listen_for_param_list_messages(&connection);
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

fn listen_for_param_list_messages(connection: &Box<dyn MavConnection<MavMessage> + Send + Sync>) {
    loop {
        match connection.try_recv() {
            Ok((_, msg)) => match msg {
                mavlink::ardupilotmega::MavMessage::PARAM_VALUE(data) => {
                    print_param_data(&data);
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
