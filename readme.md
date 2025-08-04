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

### 5. Read parameter / Set parameter
```sh
cargo run --example read_set_parameter
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
autopilot_system_id: 1
autopilot_component_id: 1
Sending param request read message: PARAM_REQUEST_READ(PARAM_REQUEST_READ_DATA { param_index: -1, target_system: 1, target_component: 1, param_id: [83, 73, 77, 95, 83, 80, 69, 69, 68, 85, 80, 0, 0, 0, 0, 0] })
65535
id:    SIM_SPEEDUP
value: 1

Sending param request set message: PARAM_SET(PARAM_SET_DATA { param_value: 42.0, target_system: 1, target_component: 1, param_id: [83, 73, 77, 95, 83, 80, 69, 69, 68, 85, 80, 0, 0, 0, 0, 0], param_type: MAV_PARAM_TYPE_REAL32 })
Reading updated parameter
65535
id:    SIM_SPEEDUP
value: 42
```
#### Additional info
- [Read Single Parameter](https://mavlink.io/en/services/parameter.html#read_single)
- [Write Parameters](https://mavlink.io/en/services/parameter.html#write)


### 6. Arm/Disarm drone
```sh
cargo run --example arm_disarm
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
Vehicle > autopilot_system_id: 1
Vehicle > autopilot_component_id: 1
Vehicle > DISARMED
Vehicle > DISARMED
Vehicle > DISARMED
Vehicle > DISARMED
Vehicle > DISARMED
Sending ARM command: COMMAND_LONG(COMMAND_LONG_DATA { param1: 1.0, param2: 0.0, param3: 0.0, param4: 0.0, param5: 0.0, param6: 0.0, param7: 0.0, command: MAV_CMD_COMPONENT_ARM_DISARM, target_system: 1, target_component: 1, confirmation: 0 })
Vehicle > Command MAV_CMD_COMPONENT_ARM_DISARM is MAV_RESULT_ACCEPTED
Vehicle > ARMED
Vehicle > ARMED
Vehicle > ARMED
Vehicle > ARMED
Sending DISARM command: COMMAND_LONG(COMMAND_LONG_DATA { param1: 0.0, param2: 0.0, param3: 0.0, param4: 0.0, param5: 0.0, param6: 0.0, param7: 0.0, command: MAV_CMD_COMPONENT_ARM_DISARM, target_system: 1, target_component: 1, confirmation: 0 })
Vehicle > ARMED
Vehicle > Command MAV_CMD_COMPONENT_ARM_DISARM is MAV_RESULT_ACCEPTED
Vehicle > DISARMED
Vehicle > DISARMED
Vehicle > DISARMED
Vehicle > DISARMED
Vehicle > DISARMED
```
#### Additional info
- [Command Protocol](https://mavlink.io/en/services/command.html#command-protocol)
- [MAV_CMD_COMPONENT_ARM_DISARM](https://mavlink.io/en/messages/common.html#MAV_CMD_COMPONENT_ARM_DISARM)
- [MAV_MODE_FLAG](https://mavlink.io/en/messages/common.html#MAV_MODE_FLAG)

### 6. Change flight mode
```sh
cargo run --example fligh_mode
``` 
#### Example output
```
Started...
Connected to tcpout:127.0.0.1:14550
Vehicle > autopilot_system_id: 1
Vehicle > autopilot_component_id: 1
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Sending set flight mode GUIDED command: COMMAND_LONG(COMMAND_LONG_DATA { param1: 1.0, param2: 4.0, param3: 0.0, param4: 0.0, param5: 0.0, param6: 0.0, param7: 0.0, command: MAV_CMD_DO_SET_MODE, target_system: 1, target_component: 1, confirmation: 0 })
Vehicle > flight mode: STABILIZE(0)
Vehicle > Command MAV_CMD_DO_SET_MODE is MAV_RESULT_ACCEPTED
Vehicle > flight mode: GUIDED(4)
Vehicle > flight mode: GUIDED(4)
Vehicle > flight mode: GUIDED(4)
Vehicle > flight mode: GUIDED(4)
Vehicle > flight mode: GUIDED(4)
Sending set flight mode STABILIZE command: COMMAND_LONG(COMMAND_LONG_DATA { param1: 1.0, param2: 0.0, param3: 0.0, param4: 0.0, param5: 0.0, param6: 0.0, param7: 0.0, command: MAV_CMD_DO_SET_MODE, target_system: 1, target_component: 1, confirmation: 0 })
Vehicle > flight mode: GUIDED(4)
Vehicle > Command MAV_CMD_DO_SET_MODE is MAV_RESULT_ACCEPTED
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
Vehicle > flight mode: STABILIZE(0)
```
#### Additional info
- [Command Protocol](https://mavlink.io/en/services/command.html#command-protocol)
- [MAV_CMD_DO_SET_MODE](https://mavlink.io/en/messages/common.html#MAV_CMD_DO_SET_MODE)

### 7. Upload mission with waypoints
```sh
cargo run --example mission
``` 
#### Example output
```
GSC > Started...
GSC > Connected to tcpout:127.0.0.1:14550
Vehicle > autopilot_system_id: 1
Vehicle > autopilot_component_id: 1
GSC > Sending: MISSION_COUNT(MISSION_COUNT_DATA { count: 7, target_system: 1, target_component: 1 })
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 0, target_system: 255, target_component: 0 }
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 1, target_system: 255, target_component: 0 }
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 2, target_system: 255, target_component: 0 }
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 3, target_system: 255, target_component: 0 }
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 4, target_system: 255, target_component: 0 }
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 5, target_system: 255, target_component: 0 }
Vehicle > misssion request MISSION_REQUEST_DATA { seq: 6, target_system: 255, target_component: 0 }
Vehicle > mission ack, MAV_MISSION_ACCEPTED
GSC > request mission list from the vehicle
GSC > Sending: MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA { target_system: 1, target_component: 1 })
Vehicle > mission count 7
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 0, target_system: 1, target_component: 1 })
Vehicle > mission item, x:-353632624, y:1491652370, z:584.08, command:MAV_CMD_NAV_WAYPOINT,
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 1, target_system: 1, target_component: 1 })
Vehicle > mission item, x:0, y:0, z:50.0, command:MAV_CMD_NAV_TAKEOFF,
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 2, target_system: 1, target_component: 1 })
Vehicle > mission item, x:-353612608, y:1491651712, z:100.0, command:MAV_CMD_NAV_WAYPOINT,
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 3, target_system: 1, target_component: 1 })
Vehicle > mission item, x:-353622592, y:1491661696, z:100.0, command:MAV_CMD_NAV_WAYPOINT,
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 4, target_system: 1, target_component: 1 })
Vehicle > mission item, x:-353632576, y:1491671808, z:100.0, command:MAV_CMD_NAV_WAYPOINT,
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 5, target_system: 1, target_component: 1 })
Vehicle > mission item, x:-353642592, y:1491681664, z:100.0, command:MAV_CMD_NAV_WAYPOINT,
GSC > Sending item request MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA { seq: 6, target_system: 1, target_component: 1 })
Vehicle > mission item, x:-353652576, y:1491691776, z:100.0, command:MAV_CMD_NAV_WAYPOINT,
```
#### Additional info
- [Upload mission](https://mavlink.io/en/services/mission.html#uploading_mission)
