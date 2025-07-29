use std::{sync::Arc, thread, time::Duration};

use mavlink::{
    MavConnection,
    ardupilotmega::{MavMessage, MavModeFlag},
    error::MessageReadError,
};

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";
const ARM_PARAM: f32 = 1f32;
const DISARM_PARAM: f32 = 0f32;

fn main() {
    println!("Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();
    println!("Connected to {}", SERVER_ADDRESS);
    let (autopilot_system_id, autopilot_component_id) = fetch_system_id(&connection);
    println!("Vehicle > autopilot_system_id: {autopilot_system_id}");
    println!("Vehicle > autopilot_component_id: {autopilot_component_id}");
    let arc = Arc::new(connection);
    listen_for_arm_status(arc.clone());

    thread::sleep(Duration::from_secs(5));
    let header = mavlink::MavHeader::default();
    let arm_command_message = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(
        mavlink::ardupilotmega::COMMAND_LONG_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
            command: mavlink::ardupilotmega::MavCmd::MAV_CMD_COMPONENT_ARM_DISARM,
            param1: ARM_PARAM,
            ..Default::default()
        },
    );
    println!("Sending ARM command: {:?}", arm_command_message);
    arc.send(&header, &arm_command_message).unwrap();
    thread::sleep(Duration::from_secs(5));
    let disarm_command_message = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(
        mavlink::ardupilotmega::COMMAND_LONG_DATA {
            target_system: autopilot_system_id,
            target_component: autopilot_component_id,
            command: mavlink::ardupilotmega::MavCmd::MAV_CMD_COMPONENT_ARM_DISARM,
            param1: DISARM_PARAM,
            ..Default::default()
        },
    );
    println!("Sending DISARM command: {:?}", disarm_command_message);
    arc.send(&header, &disarm_command_message).unwrap();
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

fn listen_for_arm_status(connection: Arc<Box<dyn MavConnection<MavMessage> + Send + Sync>>) {
    thread::spawn({
        move || loop {
            match connection.try_recv() {
                Ok((_, msg)) => match msg {
                    mavlink::ardupilotmega::MavMessage::HEARTBEAT(data) => {
                        let is_armed = data
                            .base_mode
                            .contains(MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED);
                        if is_armed {
                            println!("Vehicle > ARMED");
                        } else {
                            println!("Vehicle > DISARMED");
                        }
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
