use std::{thread, time::Duration};

use mavlink::{MavConnection, ardupilotmega::MavMessage};

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
    loop {
        println!(
            "Sending param request list message: {:?}",
            param_request_list_message
        );
        connection
            .send(&header, &param_request_list_message)
            .unwrap();
        listen_for_param_list_messages(&connection);
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

fn listen_for_param_list_messages(connection: &Box<dyn MavConnection<MavMessage> + Send + Sync>) {
    loop {
        match connection.try_recv() {
            Ok((_, msg)) => match msg {
                mavlink::ardupilotmega::MavMessage::PARAM_REQUEST_LIST(data) => {
                    println!("PARAM_REQUEST_LIST: {:?}", data);
                }
                _ => {
                    // ignore other messages
                }
            },
            Err(e) => {
                println!("recv error: {e:?}");
                panic!()
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
