## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device



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
  4. prop_o_matic_manager_device class


# prop_o_matic_manager_device class
Learn technical details about the prop_o_matic_manager_device class. 
On this page
Allows customization of the Prop-o-Matic weapon functions and how the game reacts to players using it.
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
`BeginEnteringDisguiseEvent` | `listenable(payload)` |  Signaled when an `agent` begins entering a disguise. Sends the `agent` that began entering the disguise.  
`ExitingDisguiseEvent` | `listenable(payload)` |  Signaled when an `agent` exits a disguise. Sends the `agent` that exited the disguise.  
`FinishEnteringDisguiseEvent` | `listenable(payload)` |  Signaled when an `agent` finishes entering a disguise. Sends the `agent` that finished entering the disguise.  
`PingAllPlayerPropsEvent` | `listenable(payload)` |  Signaled when all player props have been pinged.  
`PingPlayerPropEvent` | `listenable(payload)` |  Signaled when a player prop has been pinged. Sends the `agent` that was pinged.  
### Functions
Function Name | Description  
---|---  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`IsPlayerProp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/isplayerprop) |  Returns whether a player is currently hiding or not.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`PingPlayerProp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/pingplayerprop) |  Manually ping a specific player if they are currently a prop.  
[`PingPlayerProps`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/pingplayerprops) |  Manually ping all players that are currently hiding as props.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`SetPingFrequency`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/setpingfrequency) |  Adjust the ping time.  
[`SetPingProps`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/setpingprops) |  Toggle Pinging props on/off.  
[`SetShowPropPingCooldown`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/setshowproppingcooldown) |  Toggle showing the prop ping cooldown.  
[`SetShowPropsRemaining`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device/setshowpropsremaining) |  Toggle showing the props remaining on the UI.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/prop_o_matic_manager_device#functions)




