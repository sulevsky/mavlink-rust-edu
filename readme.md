# Simple educational project to connect PC to Ardupilot with Mavlink and Rust

## Requirements
- Cargo and Rust
- Build Ardupilot simulator
- Mavproxy

## Prerequisite for every examle
1. Run Ardupilot simulator
```sh
./arducopter -w -S --model + --defaults parameters/copter.parm -I0
```
2. Run Mavproxy
```sh
mavproxy.py --master tcp:127.0.0.1:5760 --out tcpin:127.0.0.1:14550 --console --map
```

## Components
```mermaid
flowchart TD
    A["Ardupilot simulator"]
    B["Mavproxy"]
    C["Rust mavlink example"]
    A <-->|Port 5760|B
    B --> |Port 14550|C
```

## Examples
1. Connect to the vehicle
```sh
cargo run --example connect
```
Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
No new messages
Received heartbeat, header: MavHeader { system_id: 1, component_id: 1, sequence: 162 }, HEARTBEAT_DATA { custom_mode: 0, mavtype: MAV_TYPE_QUADROTOR, autopilot: MAV_AUTOPILOT_ARDUPILOTMEGA, base_mode: MavModeFlag(MAV_MODE_FLAG_MANUAL_INPUT_ENABLED | MAV_MODE_FLAG_STABILIZE_ENABLED | MAV_MODE_FLAG_CUSTOM_MODE_ENABLED), system_status: MAV_STATE_STANDBY, mavlink_version: 3 }
Received heartbeat, header: MavHeader { system_id: 1, component_id: 1, sequence: 15 }, HEARTBEAT_DATA { custom_mode: 0, mavtype: MAV_TYPE_QUADROTOR, autopilot: MAV_AUTOPILOT_ARDUPILOTMEGA, base_mode: MavModeFlag(MAV_MODE_FLAG_MANUAL_INPUT_ENABLED | MAV_MODE_FLAG_STABILIZE_ENABLED | MAV_MODE_FLAG_CUSTOM_MODE_ENABLED), system_status: MAV_STATE_STANDBY, mavlink_version: 3 }
N
```