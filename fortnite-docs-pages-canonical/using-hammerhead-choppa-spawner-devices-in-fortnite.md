## https://dev.epicgames.com/documentation/en-us/fortnite/using-hammerhead-choppa-spawner-devices-in-fortnite

# Hammerhead Choppa Spawner Devices
Let players move across your island in a flying tactical vehicle.
![Hammerhead Choppa Spawner Devices](https://dev.epicgames.com/community/api/documentation/image/c53cf5ed-81a9-4c67-8caa-166320285b24?resizing_type=fill&width=1920&height=335)
The **Hammerhead Choppa Spawner** device is a five-seater helicopter vehicle designed for squad mobility, rotation, and tactical positioning. This device doesn’t feature a weapons system, but is perfect for moving a squad quickly across a large map and gaining the higher ground for a strategic retreat to escape dangerous, low-ground situations.
For help on how to find the **Hammerhead Choppa Spawner** device, see **[Using Devices](https://dev.epicgames.com/documentation/en-us/fortnite/using-devices-in-fortnite)**.
##  Contextual Filtering
Some devices are affected by a feature called contextual filtering. This feature hides or displays options depending on the values selected for certain related options. This feature reduces clutter in the Customize panel and make options easier to manage and navigate.
However, it may not be easy to recognize which options or values trigger contextual filtering. To help you identify them, in our device docs we use italic for any values that trigger contextual filtering. All options will be listed, including those affected by contextual filtering. If they are hidden or displayed based on a specific option’s value, there will be a note in the Description field for that option.
##  Device Options
This device has some basic functionality, like determining whether the device is enabled at the start of a game. Additionally, there are some advanced options, like choosing which teams or classes can use the vehicles spawned by this device.
You can configure this device with the following options.
Default values are in bold. Values that trigger contextual filtering are italic.
|  |
---|---|---
Option |  Value |  Description
**Enabled During Phase** |  None, **Always** , Pre-Game Only, Gameplay Only, Create Only |  Determines which game phase the device is enabled in. If set to **Creative Only** , the device is only enabled while you are editing your island.
**Vehicle Indestructible** |  _On_ , **Off** |  Determines if the vehicle can be destroyed by damage. When this option is set to **On** , the Vehicle Health option does not display.
**Enable Respawn** |  **On** , _Off_ |  Determines if the vehicle respawns after being destroyed. When this option is set to No, all respawn options below do not display.
**Respawn Time** |  **Instant** , Select a time |  When the vehicle is destroyed, respawn it after this delay. (The first vehicle spawned does not use this delay.)
**Respawn Vehicle when Enabled** |  Only If Needed, **Yes** , No |  Determines when to enable respawning the vehicle.
  * **Yes** means the device spawns a vehicle when enabled.
  * **Only If Needed** means the device doesn’t reset an existing vehicle.
  * **No** means the device does not spawn a vehicle.

**Vehicle Health** |  Select an amount |  Determines how much damage the vehicle can take before it is destroyed.
**Destroy Vehicle when Disabled** |  **On** , Off |  When this option is set to **On** , the device destroys an already spawned vehicle when this spawner is disabled.
**Fuel Consumption** |  _On_ , **Off** |  Determines if the spawned vehicle uses fuel. When this option is set to O**n** , two additional fuel options display.
**Random Starting Fuel** |  **On** , _Off_ |  Spawns the vehicle with a random percentage of fuel. When this option is set to **Off** , minimum and maximum random fuel options do not display.
**Min Random Starting Fuel** |  Select an amount |  This option only displays when F**uel Consumption** is set off to O**n**. Determines the minimum random fuel percentage the vehicle spawns with. If the max fuel is less than or equal to min, the vehicle spawns with default fuel.
**Max Random Starting Fuel** |  Select an amount |  This option only displays when Fuel Consumption is set off to **On**. Determines the maximum random fuel percentage the vehicle spawns with. If the max fuel is less than or equal to min, the vehicle spawns with default fuel.
**Fuel Use Multiplier** |  Select an amount |  Controls how quickly the vehicle uses fuel while driving.
**Destroy When Stuck Underwater** |  **On** , _Off_ |  Determines if the vehicle destroys itself when stuck underwater. When set to **Off** , the Water Destruction Timer option does not display.
**Water Destruction Timer** |  Select an amount |  Determines the delayed amount of time before the vehicle destroys itself when stuck underwater.
**Activating Team** |  **Any** , Select a team number |  The team this device belongs to.
**Allowed Class** |  No Class, **All** , Any, Select a number |  Determines which class has access to the vehicle.
  * **All** means all classes, including players without a class are affected.
  * **Any Class** means players with a class assigned, regardless of which class they belong to can use the device.
  * **No Class** means players without any class assigned can use the device.

**Visible During Game** |  **On** , Off |  Determines whether the device is visible during the game. This affects its collision properties.
**Skydive On Exit** |  **Yes** , No |  Determines whether players can skydive out of the vehicle. (Skydiving players can use the glider regardless of island settings.)
##  Direct Event Binding
Following are the direct event binding options for this device.
###  Functions
A function listens for an event on a device then performs an action.
  * For any function, click the **option** , then **Select Device** to access and select from the **Device** dropdown menu.

  * Once you've selected a device, click **Select Event** to bind the device to an event that will trigger the function.

  * If more than one device or event triggers a function, click the **Add** button to add a line and repeat these steps.

|
---|---
Option |  Description
**Enable When Receiving From** |  Enables the vehicle spawner, allowing it to spawn vehicles.
**Disable When Receiving From** |  Disables the vehicle spawner, stopping it from spawning any more vehicles.
**Respawn Vehicle When Receiving From** |  Spawns the vehicle (destroying the existing vehicle if it still exists).
**Destroy Vehicle When Receiving From** |  If the vehicle that this spawner created still exists, destroy it.
**Assign Driver When Receiving From** |  Seats the player that instigated the message as the spawned vehicle’s driver.
**Apply Off Road Tires When Receiving From** |  Applies off-road tires to the vehicle if the vehicle can revive them.
**Remove Tire Modification When Receiving From** |  Reverts to road tires on the vehicle if the vehicle can receive them.
**Pop All Tires When Receiving From** |  Pops all tires on the vehicle if the vehicle has tires to pop.
**Repair All Tires When Receiving From** |  Repairs all of the tires on the vehicle if the vehicle has tires to repair.
**Repair Vehicle When Receiving From** |  Restores the spawned vehicle to full health.
###  Events
An event tells another device when to perform a function.
  * For any event, click the option, then **Select Device** to access and select from the **Device** dropdown menu.

  * Once you've selected a device, click Select **Function** to bind this event to a function for that device.

  * If more than one function is triggered by the event, click the **Add** button to add a line and repeat these steps.

|
---|---
Option |  Description
**On Players Enter Vehicle Send Event To** |  When a player enters the spawned vehicle, an event is sent to the selected device.
**On Players Exit Vehicle Send Event To** |  When a player exits the vehicle, an event is sent to the selected device.
**On Vehicle Spawns Send Event To** |  When the vehicle spawns, an event is sent to the selected device.
**On Vehicle Is Destroyed Send Event To** |  When the vehicle is destroyed, an event is sent to the selected device.
