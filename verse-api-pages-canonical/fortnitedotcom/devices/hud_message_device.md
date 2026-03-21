## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device



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
  4. hud_message_device class


# hud_message_device class
Learn technical details about the hud_message_device class. 
On this page
Used to show custom HUD messages to one or more `agent`s.
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
`ClearAllMessagesEvent` | `listenable(payload)` |  Called when all queued _Messages_ from all players that are affected by this HUD Message Device have been cleared.  
`HideMessageEvent` | `listenable(payload)` |  Called when a _Message_ has been Hidden on-screen. Returns an `Agent` if it was Hidden from a specified `Agent`'s screen.  
`ShowMessageEvent` | `listenable(payload)` |  Called when a _Message_ has been Shown on-screen. Returns an `Agent` if it was Shown on a specified `Agent`'s screen.  
### Functions
Function Name | Description  
---|---  
[`ClearAllMessages`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/clearallmessages) |  Clears all queued _Messages_ from all players that are affected by this HUD Message Device.  
[`GetDisplayTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/getdisplaytime) |  Returns the time (in seconds) for which the HUD message will be displayed. `0.0` means the message is displayed persistently.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`Hide`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/hide) |  Hides the HUD message.  
[`Hide`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/hide-1) |  Hides the currently set HUD _Message_ on `Agent`s screen. Use this when the device is setup to target specific `agent`s.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`SetDisplayTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/setdisplaytime) |  Sets the time (in seconds) the HUD message will be displayed. `0.0` will display the HUD message persistently.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`SetText`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/settext) |  Sets the _Message_ to be displayed when the HUD message is activated. `Text` is clamped to 150 characters.  
[`Show`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/show) |  Shows the currently set HUD _Message_ on `Agent`s screen. Will replace any previously active message. Use this when the device is setup to target specific `agent`s.  
[`Show`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/show-1) |  Shows the currently set _Message_ HUD message on screen. Will replace any previously active message.  
[`Show`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/show-2) |  Displays a Custom message to a specific _Agent_ that you define.Setting _DisplayTime_ to `0.0` will display the HUD message persistently.If not defined, or less than `0.0` the message will show for the time set on the device.  
[`Show`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device/show-3) |  Displays a Custom message that you define for all PlayersSetting _DisplayTime_ to `0.0` will display the HUD message persistently.If not defined, or less than `0.0` the message will show for the time set on the device.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hud_message_device#functions)




