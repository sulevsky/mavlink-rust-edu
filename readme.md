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
### 1. Connect to the vehicle
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

### 2. Receive and parse messages
```sh
cargo run --example receive_message
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
HEARTBEAT, system status: MAV_STATE_STANDBY
ATTITUDE, roll: 0.06992559, , pitch: 0.058762293, yaw: -5.9922223
GLOBAL_POSITION_INT, lat: -35.3632622, lon: 149.1652375, alt: 584.08, relative_alt: -0.006
No new messages
ATTITUDE, roll: 0.069812946, , pitch: 0.058732785, yaw: -5.9924803
GLOBAL_POSITION_INT, lat: -35.3632622, lon: 149.1652375, alt: 584.08, relative_alt: -0.006
...
```

### 3. Send a heartbeat message from Rust
```sh
cargo run --example send_heartbeat
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
Sending heartbeat message: HEARTBEAT(HEARTBEAT_DATA { custom_mode: 0, mavtype: MAV_TYPE_GCS, autopilot: MAV_AUTOPILOT_INVALID, base_mode: MavModeFlag(MAV_MODE_FLAG_SAFETY_ARMED), system_status: MAV_STATE_ACTIVE, mavlink_version: 0 })
Sending heartbeat message: HEARTBEAT(HEARTBEAT_DATA { custom_mode: 0, mavtype: MAV_TYPE_GCS, autopilot: MAV_AUTOPILOT_INVALID, base_mode: MavModeFlag(MAV_MODE_FLAG_SAFETY_ARMED), system_status: MAV_STATE_ACTIVE, mavlink_version: 0 })
...
```
#### Additional info
- [MAVLink System and Component ID Assignment](https://mavlink.io/en/services/mavlink_id_assignment.html)
- [Heartbeat/Connection Protocol](https://mavlink.io/en/services/heartbeat.html)

### 4. Request parameters
```sh
cargo run --example request_parameters
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
autopilot_system_id: 1
autopilot_component_id: 1
Sending param request list message: PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA { target_system: 1, target_component: 1 })
0
id:    FORMAT_VERSION
value: 120

1
id:    SYSID_THISMAV
value: 1

2
id:    SYSID_MYGCS
value: 255

3
id:    PILOT_THR_FILT
value: 0

4
id:    PILOT_TKOFF_ALT
value: 0

5
id:    PILOT_THR_BHV
value: 0

6
id:    TELEM_DELAY
value: 0
...
```
#### Additional info
- [Parameter Protocol](https://mavlink.io/en/services/parameter.html)

#### 5. Read parameter / Set parameter

## TODO
[IN-PROGRESS]5. Change flight mode
[IN-PROGRESS]5. Arm/Disarm drone
[IN-PROGRESS]5. Send RC overrides
[IN-PROGRESS]5. Upload mission with waypoints
