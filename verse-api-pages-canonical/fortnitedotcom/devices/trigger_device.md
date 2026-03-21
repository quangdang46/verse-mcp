## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device



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
  4. trigger_device class


# trigger_device class
Learn technical details about the trigger_device class. 
On this page
Used to relay events to other linked devices.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description  
---|---  
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.  
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.  
[`trigger_base_device`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device) |  Base class for various specialized trigger devices. See also: _`trigger_device` _ `perception_trigger_device` * `attribute_evaluator_device`  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`TriggeredEvent` | `listenable(payload)` |  Signaled when an `agent` triggers this device. Sends the `agent` that used this device. Returns `false` if no `agent` triggered the action (ex: it was triggered through code).  
### Functions
Function Name | Description  
---|---  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/disable) |  Disables this device.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/enable) |  Enables this device.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetMaxTriggerCount`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/getmaxtriggercount) |  Gets the maximum amount of times this device can trigger.
  * `0` indicates no limit on trigger count.

  
[`GetResetDelay`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/getresetdelay) |  Gets the time (in seconds) before the device can be triggered again (if `MaxTrigger` count allows).  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`GetTransmitDelay`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/gettransmitdelay) |  Gets the time (in seconds) which must pass after triggering, before this device informs other external devices that it has been triggered.  
[`GetTriggerCountRemaining`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/gettriggercountremaining) |  Returns the number of times that this device can still be triggered before hitting `GetMaxTriggerCount`. Returns `0` if `GetMaxTriggerCount` is unlimited.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Reset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/reset) |  Resets the number of times this device has been activated. This will set `GetTriggerCountRemaining` back to `0`  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`SetMaxTriggerCount`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/setmaxtriggercount) |  Sets the maximum amount of times this device can trigger.
  * `0` can be used to indicate no limit on trigger count.
  * `MaxCount` is clamped between [0,20].

  
[`SetResetDelay`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/setresetdelay) |  Sets the time (in seconds) after triggering, before the device can be triggered again (if `MaxTrigger` count allows).  
[`SetTransmitDelay`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_base_device/settransmitdelay) |  Sets the time (in seconds) which must pass after triggering, before this device informs other external devices that it has been triggered.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`Trigger`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device/trigger) |  Triggers this device with `Agent` being passed as the `agent` that triggered the action. Use an `agent` reference when this device is setup to require one (for instance, you want to trigger the device only with a particular `agent`.  
[`Trigger`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device/trigger-1) |  Triggers this device, causing it to activate its `TriggeredEvent` event.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/trigger_device#functions)




