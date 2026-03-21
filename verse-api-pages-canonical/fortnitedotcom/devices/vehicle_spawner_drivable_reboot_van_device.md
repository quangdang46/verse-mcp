## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device



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
  4. vehicle_spawner_drivable_reboot_van_device class


# vehicle_spawner_drivable_reboot_van_device class
Learn technical details about the vehicle_spawner_drivable_reboot_van_device class. 
On this page
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description  
---|---  
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.  
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.  
[`vehicle_spawner_device`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_device) |  Base class for various specialized vehicle spawners which allow specific vehicle types to be spawned and configured with specialized options.  
## Exposed Interfaces
This class exposes the following interfaces:
Name | Description  
---|---  
[`reboot_van_interface`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface) |   
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`AgentEntersVehicleEvent` | `listenable(payload)` |  Signaled when an `agent` enters the vehicle. Sends the `agent` that entered the vehicle.  
`AgentExitsVehicleEvent` | `listenable(payload)` |  Signaled when an `agent` exits the vehicle. Sends the `agent` that exited the vehicle.  
`CanPurchaseRebootCard` | `?logic` |  Determines if players can purchase an eliminated player's reboot card.  
`DecayRateMultiplier` | `??float` |  Multiplier on the decay rate of reboot progress. Clamped between `0.1` and `2.0`.
  * Only used if `RebootProgressDecay` is set to `Custom Decay`.

  
`DestroyedEvent` | `listenable(payload)` |  Signaled when a vehicle is destroyed.  
`PurchaseRebootCardOptions` | `??reboot_card_purchase_options` |  Purchase reboot card options.
  * Only used if `CanPurchaseRebootCard` is `true`.

  
`RebootCardPurchaseEvent` | `listenable(payload)` |  Triggers when a player purchases a Reboot Card.
  * `agent` is the player that purchased the Reboot Card.

  
`RebootEvent` | `listenable(payload)` |  Triggers when Reboot Van has finished rebooting a set of players.
  * `agent`is the player that started the reboot.

  
`RebootProgressDecay` | `?reboot_progress_decay_behavior` |  How quickly reboot progress decays when nobody is interacting with the Reboot Van. Custom Decay - Set a custom multiplier on the decay rate. Instant Reset - Instantly reset progress to zero. Battle Royale - Use Battle Royale's decay rate.  
`RechargeCompleteEvent` | `listenable(payload)` |  Triggers when Reboot Van has finished recharging.
  * `agent` is the last interacting player.

  
`RechargeTimer` | `?float` |  The remaining time (in seconds) on the recharge timer. Clamped between `0.0` and `3600.0`.
  * If there is no active timer, getting returns `0.0`.
  * If there is no active timer, setting does nothing.

  
`RechargeTimerLength` | `?float` |  The length of the recharge timer in seconds, regardless of the timer's current state. Clamped between `0.0` and `3600.0`.  
`ReviveCompleteEvent` | `listenable(payload)` |  Triggers when Reboot Van has finished reviving a player from DBNO.
  * `agent` is the player that was just revived.

  
`SpawnedEvent` | `listenable(payload)` |  Signaled when a vehicle is spawned or respawned by this device. Sends the fort_vehicle who was spawned.  
`VehicleDestroyedEvent` | `listenable(payload)` |  Signaled when a vehicle is destroyed. Deprecated, use DestroyedEvent instead.  
`VehicleSpawnedEvent` | `listenable(payload)` |  Signaled when a vehicle is spawned or respawned by this device. Deprecated, use SpawnedEvent instead.  
### Functions
Function Name | Description  
---|---  
[`AssignDriver`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_device/assigndriver) |  Sets `agent` as the vehicle's driver.  
[`DestroyVehicle`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_device/destroyvehicle) |  Destroys the vehicle if it exists.  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_device/disable) |  Disables this device.  
[`DisableReboot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device/disablereboot) |  Disable the device.  
[`DisableRevive`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device/disablerevive) |  Disable the revival seat.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_device/enable) |  Enables this device.  
[`EnableReboot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device/enablereboot) |  Enable the device.  
[`EnableRevive`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device/enablerevive) |  Enable the revival seat.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`IsEnabledReboot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device/isenabledreboot) |  Succeeds if the device is enabled, fails if it's disabled.  
[`IsEnabledRevive`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device/isenabledrevive) |  Succeeds if the revival seat is enabled, fails if it's disabled.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`RespawnVehicle`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_device/respawnvehicle) |  Spawns a new vehicle. The previous vehicle will be destroyed before a new vehicle spawns.  
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
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device#inheritancehierarchy)
  * [Exposed Interfaces](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device#exposedinterfaces)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_spawner_drivable_reboot_van_device#functions)




