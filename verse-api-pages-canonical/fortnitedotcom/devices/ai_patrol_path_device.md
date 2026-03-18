## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/ai_patrol_path_device

# ai_patrol_path_device class
Learn technical details about the ai_patrol_path_device class.
Used to create patrolling behavior for guards spawned with the `guard_spawner_device`.
|
---|---
Verse `using` statement | `using { /Fortnite.com/Devices }`
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description
---|---
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description
---|---|---
`NextNodeUnreachableEvent` | `listenable(payload)` |  Signaled when a guard cannot reach the next `ai_patrol_path_device`.
`NodeReachedEvent` | `listenable(payload)` |  Signaled when a guard reaches this device.
`PatrolPathStartedEvent` | `listenable(payload)` |  Signaled when a guard starts moving on the patrol path.
`PatrolPathStoppedEvent` | `listenable(payload)` |  Signaled when a guard stops moving on the patrol path.
### Functions
Function Name | Description
---|---
[`Assign`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/ai_patrol_path_device/assign) |  Assign an AI to this patrol path.
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/ai_patrol_path_device/disable) |  Disables this device.
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/ai_patrol_path_device/enable) |  Enables this device.
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.
[`GoToNextPatrolGroup`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/ai_patrol_path_device/gotonextpatrolgroup) |  Commands patroller to follow the _Next Patrol Path Group_ instead of the default _Patrol Path Group_.
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.
