use std::{thread, time::Duration};

use mavlink::error::MessageReadError;

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";

fn main() {
    println!("Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();
    println!("Connected to {}", SERVER_ADDRESS);

    loop {
        match connection.recv() {
            Ok((_, msg)) => match msg {
                mavlink::ardupilotmega::MavMessage::HEARTBEAT(data) => {
                    println!("HEARTBEAT, system status: {:?}", data.system_status);
                }
                mavlink::ardupilotmega::MavMessage::GLOBAL_POSITION_INT(data) => {
                    println!(
                        "GLOBAL_POSITION_INT, lat: {:?}, lon: {:?}, alt: {:?}, relative_alt: {:?}",
                        (data.lat as f64) / 10_000_000.0,
                        (data.lon as f64) / 10_000_000.0,
                        (data.alt as f64) / 1_000.0,
                        (data.relative_alt as f64) / 1_000.0
                    );
                }
                mavlink::ardupilotmega::MavMessage::ATTITUDE(data) => {
                    println!(
                        "ATTITUDE, roll: {:?}, , pitch: {:?}, yaw: {:?}",
                        data.roll.to_degrees(),
                        data.pitch.to_degrees(),
                        data.yaw.to_degrees(),
                    );
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
