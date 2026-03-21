## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle



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
  4. fort_vehicle interface


# fort_vehicle interface
Learn technical details about the fort_vehicle interface. 
On this page
Main API implemented by Fortnite vehicles.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Vehicles }`  
## Exposed Interfaces
This interface exposes the following interfaces:
Name | Description  
---|---  
[`positional`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/positional) |  Implemented by objects to allow reading position information.  
[`healthful`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healthful) |  Implemented by Fortnite objects that have health state and can be eliminated.  
[`damageable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable) |  Implemented by Fortnite objects that can be damaged.  
[`game_action_causer`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/game_action_causer) |  Implemented by Fortnite objects that can be passed through game action events, such as damage and heal. For example: player, vehicle, or weapon. Event Listeners often use `game_action_causer` to pass along additional information about what weapon caused the damage. Systems will then use that information for completing quests or processing game specific event logic.  
## Members
This interface has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`Speed` | `?float` |  The current speed of the vehicle in m/s.  
`BoostRemaining` | `??float` |  The boost state of the vehicle. If the vehicle uses boost, this value will be between 0.0 and `BoostCapacity`. Otherwise, this value will be false.  
`BoostCapacity` | `??float` |  The maximum boost capacity of the vehicle. If the vehicle uses boost, this value will be between 1.0 and Inf. Otherwise, this value will be false.  
### Functions
Function Name | Description  
---|---  
[`IsOnGround`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/isonground) |  Succeeds if this `fort_vehicle` is standing on ground.  
[`IsInAir`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/isinair) |  Succeeds if this `fort_vehicle` is standing in air.  
[`IsInWater`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/isinwater) |  Succeeds if this `fort_vehicle` is standing in water.  
[`GetPassengers`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/getpassengers) |  Returns an array with all the passengers of the vehicle.  
[`GetFuelRemaining`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/getfuelremaining) |  Returns the fuel state of the vehicle. If the vehicle uses fuel, this value will be between 0.0 and `GetFuelCapacity`. Otherwise, this value will be -1.0.  
[`GetFuelCapacity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/getfuelcapacity) |  Returns the maximum fuel capacity of the vehicle. If the vehicle uses fuel, this value will be between 1.0 and Inf. Otherwise, this value will be -1.0.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle/teleportto) |  Teleports the `fort_vehicle` to the specified `Position` and `Rotation`.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Exposed Interfaces](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle#exposedinterfaces)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle#functions)




