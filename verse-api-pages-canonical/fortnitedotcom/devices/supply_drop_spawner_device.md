## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device



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
  4. supply_drop_spawner_device class


# supply_drop_spawner_device class
Learn technical details about the supply_drop_spawner_device class. 
On this page
Used to spawn and configure an aerial supply drop that can provide players with customized weapons/supplies.
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
`BalloonPoppedEvent` | `listenable(payload)` |  Signaled when the balloon on the supply crate is popped. Sends the `?agent` that popped the balloon. If no `agent` popped the balloon returns `false`.  
`DestroyCrateEvent` | `listenable(payload)` |  Signaled when the supply crate is destroyed. Sends the destroying `agent`. If no `agent` destroyed the crate then `false` is returned.  
`LandingEvent` | `listenable(payload)` |  Signaled when the supply crate lands for the first time.  
`OpenedEvent` | `listenable(payload)` |  Signaled when the supply crate is opened. Sends the `agent` that opened the crate.  
### Functions
Function Name | Description  
---|---  
[`DestroyBalloon`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/destroyballoon) |  Destroys the balloon and causes the supply crate to freefall.  
[`DestroySpawnedDrops`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/destroyspawneddrops) |  Destroys supply drops spawned by this device.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`Lock`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/lock) |  Locks the supply crate so `agent`s cannot open it.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Open`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/open) |  Opens the supply crate, ignoring the locked or unlocked state. `Agent` acts as the instigator of the open action.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`Spawn`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/spawn) |  Spawns a supply drop provided one hasn't already spawned. _Owning Team_ is set to `Agent`'s team.  
[`Spawn`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/spawn-1) |  Spawns a supply drop provided one hasn't already spawned.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`Unlock`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device/unlock) |  Unlocks the supply crate so `agent`s can open it.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/supply_drop_spawner_device#functions)




