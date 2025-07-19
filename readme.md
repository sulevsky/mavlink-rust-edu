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
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
No new messages
Received heartbeat, header: MavHeader { system_id: 1, component_id: 1, sequence: 162 }, HEARTBEAT_DATA { custom_mode: 0, mavtype: MAV_TYPE_QUADROTOR, autopilot: MAV_AUTOPILOT_ARDUPILOTMEGA, base_mode: MavModeFlag(MAV_MODE_FLAG_MANUAL_INPUT_ENABLED | MAV_MODE_FLAG_STABILIZE_ENABLED | MAV_MODE_FLAG_CUSTOM_MODE_ENABLED), system_status: MAV_STATE_STANDBY, mavlink_version: 3 }
Received heartbeat, header: MavHeader { system_id: 1, component_id: 1, sequence: 15 }, HEARTBEAT_DATA { custom_mode: 0, mavtype: MAV_TYPE_QUADROTOR, autopilot: MAV_AUTOPILOT_ARDUPILOTMEGA, base_mode: MavModeFlag(MAV_MODE_FLAG_MANUAL_INPUT_ENABLED | MAV_MODE_FLAG_STABILIZE_ENABLED | MAV_MODE_FLAG_CUSTOM_MODE_ENABLED), system_status: MAV_STATE_STANDBY, mavlink_version: 3 }
N
...
```

2. Receive and parse messages
```sh
cargo run --example receive_message
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
ATTITUDE, roll: -0.05967511, , pitch: -0.07107734, yaw: -8.010325
GLOBAL_POSITION_INT, lat: -35.363262, , lon: 149.1652373, , alt: 584.08, relative_alt: -0.006
No new messages
ATTITUDE, roll: -0.059504297, , pitch: -0.07090219, yaw: -8.010025
...
```

[IN-PROGRESS]3. Send heartbeat
[IN-PROGRESS]4. Request parameters
[IN-PROGRESS]5. Change flight mode
[IN-PROGRESS]5. Arm/Disarm drone
[IN-PROGRESS]5. Send RC overrides
[IN-PROGRESS]5. Upload mission with waypoints
