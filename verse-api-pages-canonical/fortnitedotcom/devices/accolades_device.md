## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device



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
  4. accolades_device class


# accolades_device class
Learn technical details about the accolades_device class. 
On this page
Used to set up islands so players will earn Battle Pass XP when they interact with your island. Accolades are achievements or accomplishments that players can complete to earn XP.
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
`TestAwardEvent` | `listenable(payload)` |  Signaled when testing the accolade to make sure it is awarded as expected. Only signals within unpublished island environments. Sends the `agent` receiving the achievement.  
### Functions
Function Name | Description  
---|---  
[`Award`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device/award) |  Awards the XP to `agent`.  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device/disable) |  Disables this device.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device/enable) |  Enables this device.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/accolades_device#functions)






---
