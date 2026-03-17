## https://dev.epicgames.com/documentation/en-us/fortnite/using-player-checkpoint-devices-in-fortnite-creative



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
  4. Player Checkpoint Devices


# Player Checkpoint Devices
This device sets a player's spawn point when activated, and can also be used to clear player inventories. 
![Player Checkpoint Devices](https://dev.epicgames.com/community/api/documentation/image/f2a04700-1ae3-4c76-9beb-e386c3897857?resizing_type=fill&width=1920&height=335)
On this page
The **Player Checkpoint Pad** sets a player's spawn point when activated and can also be used to clear player inventories.
For help on how to find the Player Checkpoint device, see [](https://dev.epicgames.com/documentation/assets/using-devices-in-fortnite-creative)[Using Devices](https://dev.epicgames.com/documentation/en-us/fortnite/using-devices-in-fortnite). 
##  Device Options 
This device has some basic functionality, like playing sound effects and resetting inventories.
The default values are bold.
You can configure this device with the following options.
Option  |  Value  |  Description   
---|---|---  
**Visible in Game** |  **On** , Off  |  Determines whether the device is visible during the game. This does affect its collision properties.  
Reset Inventory |  Yes, No |  Determines whether the player's inventory is reset when they activate the checkpoint.  
**Enabled During Phase** |  None, **All** , Create Only, Game Countdown Only, Gameplay Only |  Determines the game phases during which the device will be enabled.  
**Activating Team** |  **Any** , Pick a team |  Determines which team can activate the device.  
**Play Activate FX** |  **On** , Off |  Determines whether the device plays VFX and SFX when stepped on.  
**Allowed Class** |  No Class, **Any** , Pick a class |  Determines which class can activate the device.  
##  Direct Event Binding 
###  Functions 
A [function](https://dev.epicgames.com/documentation/en-us/fortnite-creative/fortnite-creative-glossary#function) listens for an event on a device then performs an action.
  1. For any function, click the option, then **Select Device** to access and select from the **Device** dropdown menu.
  2. Once you've selected a device, click **Select Event** to bind the device to an event that will trigger the function.
  3. If more than one device or event triggers a function, click the **Add** button to add a line and repeat these steps.


Option  |  Description   
---|---  
**Enable When Receiving From** |  Enables the device when an event occurs.  
**Disable When Receiving From** |  Disables the device when an event occurs.  
**Activate When Receiving From** |  Registers this checkpoint to the activating player when an event occurs.  
###  Events 
An [event](https://dev.epicgames.com/documentation/en-us/fortnite-creative/fortnite-creative-glossary#event) tells another device when to perform a function.
  1. For any event, click the option, then **Select Device** to access and select from the **Device** dropdown menu.
  2. Once you've selected a device, click **Select Function** to bind this event to a function for that device.
  3. If more than one function is triggered by the event, click the **Add** button to add a line and repeat these steps.


Option  |  Description   
---|---  
**On First Activation Per Player Transmit On** |  Sends an event each time a new player activates the checkpoint for the first time.  
**On First Activation Transmit On** |  Sends an event the first time the checkpoint is activated by any player.  
  * [ devices](https://dev.epicgames.com/community/search?query=devices)
  * [ spawn](https://dev.epicgames.com/community/search?query=spawn)
  * [ checkpoint](https://dev.epicgames.com/community/search?query=checkpoint)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [ Device Options ](https://dev.epicgames.com/documentation/en-us/fortnite/using-player-checkpoint-devices-in-fortnite-creative#device-options)
  * [ Direct Event Binding ](https://dev.epicgames.com/documentation/en-us/fortnite/using-player-checkpoint-devices-in-fortnite-creative#channels)
  * [ Functions ](https://dev.epicgames.com/documentation/en-us/fortnite/using-player-checkpoint-devices-in-fortnite-creative#receivers)
  * [ Events ](https://dev.epicgames.com/documentation/en-us/fortnite/using-player-checkpoint-devices-in-fortnite-creative#transmitters)






---
