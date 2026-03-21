## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device



Table of Contents
  1. ![Epic Games](https://edc-cdn.net/assets/images/logo-epic.svg)[Developer](https://dev.epicgames.com/)
  2. [Documentation](https://dev.epicgames.com/documentation/ "Documentation")
  3. Fortnite
     * [](https://dev.epicgames.com/documentation/en-us/unreal-engine)
     * [](https://dev.epicgames.com/documentation/en-us/fortnite)
     * [](https://dev.epicgames.com/documentation/en-us/twinmotion)
     * [](https://dev.epicgames.com/documentation/en-us/metahuman)
     * [](https://dev.epicgames.com/documentation/en-us/realityscan)
     * [](https://dev.epicgames.com/documentation/en-us/realityscan-mobile)
     * [](https://dev.epicgames.com/documentation/en-us/fab)
  4. timed_objective_device class


# timed_objective_device class
Learn technical details about the timed_objective_device class. 
On this page
Configures game modes where players can start or stop timers to advance gameplay objectives, such as Attack/Defend Bomb objectives.
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
`BeganEvent` | `listenable(payload)` |  Signaled when the objective begins. Sends the `agent` that started the timer.  
`CompletedEvent` | `listenable(payload)` |  Signaled when the objective is completed. Sends the `agent` that started the timer or completed the timer by calling `Complete`.  
`EndedEvent` | `listenable(payload)` |  Signaled when the objective ends. Sends the `agent` that stopped the timer.  
`PausedEvent` | `listenable(payload)` |  Signaled when the objective is paused. Sends the `agent` that paused the timer.  
`RestartedEvent` | `listenable(payload)` |  Signaled when the objective is restarted. Sends the `agent` that restarted the timer.  
`ResumedEvent` | `listenable(payload)` |  Signaled when the objective is resumed. Sends the `agent` that resumed the timer.  
### Functions
Function Name | Description  
---|---  
[`Begin`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/begin) |  Starts the objective with `Agent` acting as the user the interacted this device.  
[`Complete`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/complete) |  Completes the objective with `Agent` acting as the user the interacted this device.  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/disable) |  Disables the objective for `Agent`.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/enable) |  Enables the objective for `Agent`.  
[`End`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/end) |  Ends the objective with `Agent` acting as the user the interacted this device.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`Hide`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/hide) |  Makes this device invisible.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Pause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/pause) |  Pauses the objective with `Agent` acting as the user the interacted this device.  
[`Restart`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/restart) |  Restarts the objective with `Agent` acting as the user the interacted this device.  
[`Resume`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/resume) |  Resumes the objective with `Agent` acting as the user the interacted this device.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`Show`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device/show) |  Makes this device visible.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timed_objective_device#functions)




