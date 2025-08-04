use std::{sync::Arc, thread, time::Duration};

use mavlink::{
    MavConnection,
    ardupilotmega::{MavCmd, MavFrame, MavMessage},
    error::MessageReadError,
};

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";

const WAYPOINTS: [(
    f32,
    f32,
    f32,
    mavlink::ardupilotmega::MavCmd,
    mavlink::ardupilotmega::MavFrame,
); 7] = [
    (
        0.0,
        0.0,
        0.0,
        MavCmd::MAV_CMD_NAV_WAYPOINT,
        MavFrame::MAV_FRAME_GLOBAL,
    ),
    (
        0.0,
        0.0,
        50.0,
        MavCmd::MAV_CMD_NAV_TAKEOFF,
        MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
    ),
    (
        -35.36125769,
        149.16517199,
        100.0,
        MavCmd::MAV_CMD_NAV_WAYPOINT,
        MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
    ),
    (
        -35.36225769,
        149.16617199,
        100.0,
        MavCmd::MAV_CMD_NAV_WAYPOINT,
        MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
    ),
    (
        -35.36325769,
        149.16717199,
        100.0,
        MavCmd::MAV_CMD_NAV_WAYPOINT,
        MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
    ),
    (
        -35.36425769,
        149.16817199,
        100.0,
        MavCmd::MAV_CMD_NAV_WAYPOINT,
        MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
    ),
    (
        -35.36525769,
        149.16917199,
        100.0,
        MavCmd::MAV_CMD_NAV_WAYPOINT,
        MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT,
    ),
];

fn main() {
    println!("GSC > Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();
    println!("GSC > Connected to {}", SERVER_ADDRESS);
    let (autopilot_system_id, autopilot_component_id) = fetch_system_id(&connection);
    println!("Vehicle > autopilot_system_id: {autopilot_system_id}");
    println!("Vehicle > autopilot_component_id: {autopilot_component_id}");
    let arc = Arc::new(connection);
    listen_for_messages(arc.clone());

    thread::sleep(Duration::from_secs(2));
    let mission_count_message = mavlink::ardupilotmega::MavMessage::MISSION_COUNT(
        mavlink::ardupilotmega::MISSION_COUNT_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
            count: WAYPOINTS.len() as u16,
        },
    );
    println!("GSC > Sending: {mission_count_message:?}");
    arc.send_default(&mission_count_message).unwrap();
    thread::sleep(Duration::from_secs(3));
    println!("GSC > request to mission list from the vehicle");
    let mission_request_list = mavlink::ardupilotmega::MavMessage::MISSION_REQUEST_LIST(
        mavlink::ardupilotmega::MISSION_REQUEST_LIST_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
        },
    );
    println!("GSC > Sending: {mission_request_list:?}");
    arc.send_default(&mission_request_list).unwrap();
    thread::sleep(Duration::from_secs(5));
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

fn create_mission_item(
    seq: u16,
    target_system: u8,
    target_component: u8,
) -> mavlink::ardupilotmega::MavMessage {
    let waypoint = WAYPOINTS[seq as usize];
    mavlink::ardupilotmega::MavMessage::MISSION_ITEM_INT(
        mavlink::ardupilotmega::MISSION_ITEM_INT_DATA {
            target_system,
            target_component,
            x: (waypoint.0 * 10000000.0) as i32,
            y: (waypoint.1 * 10000000.0) as i32,
            z: waypoint.2,
            seq,
            command: waypoint.3,
            frame: waypoint.4,
            current: 0,
            autocontinue: 0,
            ..Default::default()
        },
    )
}

fn listen_for_messages(connection: Arc<Box<dyn MavConnection<MavMessage> + Send + Sync>>) {
    thread::spawn({
        move || loop {
            match connection.try_recv() {
                Ok((header, msg)) => match msg {
                    mavlink::ardupilotmega::MavMessage::COMMAND_ACK(data) => {
                        println!("Vehicle > mcommand {:?} is {:?}", data.command, data.result);
                        println!("Vehicle > mcommand {:?} is {:?}", data.command, data.result);
                    }
                    mavlink::ardupilotmega::MavMessage::MISSION_ACK(data) => {
                        println!("Vehicle > mission ack, {:?}", data.mavtype);
                    }
                    mavlink::ardupilotmega::MavMessage::MISSION_REQUEST(data) => {
                        println!("Vehicle > misssion request {:?}", data);
                        connection
                            .send_default(&create_mission_item(
                                data.seq,
                                header.system_id,
                                header.component_id,
                            ))
                            .unwrap();
                    }
                    mavlink::ardupilotmega::MavMessage::MISSION_COUNT(data) => {
                        println!("Vehicle > mission count {:?}", data.count);
                    }
                    _ => {}
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
