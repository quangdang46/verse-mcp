## https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative



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
  4. Crash Pad Devices


# Crash Pad Devices
Place a crash pad that can bounce players and save them from fall damage. 
![Crash Pad Devices](https://dev.epicgames.com/community/api/documentation/image/a014d455-178e-4f98-91c1-129892a986d3?resizing_type=fill&width=1920&height=335)
On this page
A **Crash Pad** can bounce players into the air and save them from fall damage. It can also bounce other objects (such as vehicles, balls, or projectiles) up into the air. It is primarily used by placing it in locations where players will be expected to take a long fall without deploying the glider.
The **Crash Pad** device and the [Crash Pad](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary) item resemble each other, and work similarly once placed. However, only the Crash Pad device can be customized using the options described below, and the Crash Pad device doesn't have a time limit the way the item does.
Any player landing on a crash pad will be bounced a fixed height upward, high enough that they can deploy their glider. The bounce height is modified by any [buffs](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary) to the player’s jump height.
To find the **Crash Pad** device, see [Using Devices](https://dev.epicgames.com/documentation/en-us/fortnite/using-devices-in-fortnite).
##  Device Options 
The default bounce height of a crash pad cannot be changed. You can configure this device with the following options.
Default values are **bold**.
Option  |  Value  |  Description   
---|---|---  
**Device Health** |  **Indestructible** , Pick a number |  Determines how much damage the device can take.  
**Enabled During Phase** |  None, **Always** , Pre-Game Only, Gameplay Only, Create Only |  Enables the device during a specific phase. **Pre-Game Only** includes all phases that occur before the game starts.  
**Visual Style** |  **Default** , Original, Target |  Determines the visual style the Crash Pad uses.  
**Allow Any Launch to Send Event** |  On, **Off** |  Determines whether the Crash Pad sends an event when any object touches it (rather than just players and vehicles containing players).  
###  Physics-Enabled Options 
The following options become available when [Physics](https://dev.epicgames.com/documentation/en-us/fortnite/physics) are enabled in a project: 
Option  |  Value  |  Description   
---|---|---  
**Bounce Launch Value** |  Select a value |  Determines the impulse or velocity applied to bounced players. As long as Apply Low Gravity is set to Off, a velocity of 23 meters/second will launch a player about two floors high, and 30 meters/second will launch a player about four floors high. An impulse of 1KNs will launch a player roughly 3 meters high.  
**Impulse or Velocity** |  **Velocity** , Impulse |  Determines whether to an impulse to or directly set the velocity of an object.  
##  Event Binding 
Direct event binding allows devices to communicate directly, which makes your workflow more intuitive, and gives you more freedom to focus on your design ideas.
Below are the following direct event binding options for this device.
###  Functions 
A [function](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary) listens for an event on a device then performs an action.
  1. For any function, click the **option** , then **Select Device** to access and select from the **Device dropdown menu**.
  2. Once you've selected a device, click **Select Event** and select the event that triggers this function.
  3. If more than one device or event triggers a function, press the **Add** button to add a line and repeat these steps.


Option  |  Description   
---|---  
**Enabled When Receiving From** |  This function enables the crash pad when an event occurs.  
**Disable When Receiving From** |  This function disables the crash pad when an event occurs.  
###  Events 
Direct event binding uses events as transmitters. An event tells another device to perform a function.
  1. For any event option, click the **option** , then **Select Device** to access and select from the **Device dropdown menu**.
  2. Once you've selected a device, click **Select Function** to bind the event to a function for that device.
  3. If more than one function is triggered by the event, press the **Add** button and repeat.


Option  |  Description   
---|---  
**On Launch Send Event To** |  When any object is launched, an event is sent to the selected device which triggers the selected function.  
  * [ device](https://dev.epicgames.com/community/search?query=device)
  * [ player](https://dev.epicgames.com/community/search?query=player)
  * [ informational](https://dev.epicgames.com/community/search?query=informational)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [ Device Options ](https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative#device-options)
  * [ Physics-Enabled Options ](https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative#physics-enabledoptions)
  * [ Event Binding ](https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative#direct-event-binding)
  * [ Functions ](https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative#functions)
  * [ Events ](https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative#events)






---
