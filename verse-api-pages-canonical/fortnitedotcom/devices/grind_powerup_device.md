## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/grind_powerup_device



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
  4. grind_powerup_device class


# grind_powerup_device class
Learn technical details about the grind_powerup_device class. 
On this page
Used to let `agent`s slide on any surface with accompanying visual and audio effects.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description  
---|---  
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.  
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.  
[`powerup_device`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device) |  Base class for various powerup devices offering common events like `ItemPickedUpEvent`.  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`ItemPickedUpEvent` | `listenable(payload)` |  Signaled when the powerup is picked up by an `agent`. Sends the `agent` that picked up the powerup.  
### Functions
Function Name | Description  
---|---  
[`Despawn`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/despawn) |  Despawns this powerup from the experience.  
[`GetDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/getduration) |  Returns the _Duration_ that this powerup will be active for on any player it is applied to.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetRemainingTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/getremainingtime) |  If the `Agent` has the effect applied to them, this will return the remaining time the effect has. Returns -1.0 if the effect has an infinite duration. Returns 0.0 if the `Agent` does not have the effect applied.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`HasEffect`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/haseffect) |  Returns the `Agent` has the powerup's effect (or another of the same type) applied to them.  
[`IsSpawned`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/isspawned) |  Succeeds if the powerup is currently spawned.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Pickup`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/pickup) |  Grants this powerup to `Agent`.  
[`Pickup`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/pickup-1) |  Grants this powerup without an agent reference. Requires _Apply To_ set to _All Players_.  
[`SetDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/setduration) |  Updates the _Duration_ for this powerup, clamped to the Min and Max defined in the device. Will not apply to any currently applied effects.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`Spawn`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/powerup_device/spawn) |  Spawns the powerup into the experience so users can interact with it.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/grind_powerup_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/grind_powerup_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/grind_powerup_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/grind_powerup_device#functions)




