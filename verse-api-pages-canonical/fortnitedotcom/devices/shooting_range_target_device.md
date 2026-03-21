## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device



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
  4. shooting_range_target_device class


# shooting_range_target_device class
Learn technical details about the shooting_range_target_device class. 
On this page
A single customizable pop up target that can be hit by `agent`s to trigger various events.
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
`BullseyeHitEvent` | `listenable(payload)` |  Signaled when the target is hit in the bullseye area.  
`HitEvent` | `listenable(payload)` |  Signaled when the target is hit by an `agent`.  
`HopDownEvent` | `listenable(payload)` |  Signaled when the target moves down slightly, making it harder to hit.  
`HopUpEvent` | `listenable(payload)` |  Signaled when the target moves up slightly, making it harder to hit.  
`KnockdownEvent` | `listenable(payload)` |  Signaled when the target takes enough damage to get knocked down.  
`PopDownEvent` | `listenable(payload)` |  Signaled when the target moves from standing upright to laying flat.  
`PopUpEvent` | `listenable(payload)` |  Signaled when the target moves from laying flat to standing upright.  
### Functions
Function Name | Description  
---|---  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/disable) |  Disables this device.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/enable) |  Enables this device.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`HopDown`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/hopdown) |  Moves an active (standing upright) target down slightly, in an effort to make it harder to hit.  
[`HopUp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/hopup) |  Moves an active (standing upright) target up slightly, in an effort to make it harder to hit.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`PopDown`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/popdown) |  Causes a target to transition from standing upright (active) to lying flat (inactive).  
[`PopUp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/popup) |  Causes a target to transition from lying flat (inactive) to standing upright (active).  
[`Reset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device/reset) |  Resets the target to its initial settings.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/shooting_range_target_device#functions)




