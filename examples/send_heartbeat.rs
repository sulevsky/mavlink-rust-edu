use std::{thread, time::Duration};

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";

fn main() {
    println!("Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();
    println!("Connected to {}", SERVER_ADDRESS);
    let header = mavlink::MavHeader::default();
    let heartbeat_message =
        mavlink::ardupilotmega::MavMessage::HEARTBEAT(mavlink::ardupilotmega::HEARTBEAT_DATA {
            mavtype: mavlink::ardupilotmega::MavType::MAV_TYPE_GCS,
            autopilot: mavlink::ardupilotmega::MavAutopilot::MAV_AUTOPILOT_INVALID,
            system_status: mavlink::ardupilotmega::MavState::MAV_STATE_ACTIVE,
            ..Default::default()
        });
    loop {
        println!("Sending heartbeat message: {:?}", heartbeat_message);
        connection.send(&header, &heartbeat_message).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
