use std::{thread, time::Duration};

use mavlink::error::MessageReadError;

const SERVER_ADDRESS: &str = "tcpout:127.0.0.1:14550";

fn main() {
    println!("Started...");
    let connection = mavlink::connect(SERVER_ADDRESS).unwrap();

    println!("Connected to {}", SERVER_ADDRESS);
    loop {
        match connection.recv() {
            Ok((header, msg)) => match msg {
                mavlink::ardupilotmega::MavMessage::HEARTBEAT(data) => {
                    println!("Received heartbeat, header: {header:?}, {data:?}");
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
