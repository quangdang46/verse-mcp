## https://dev.epicgames.com/documentation/en-us/fortnite-creative/using-changing-booth-devices-in-fortnite-creative

# Changing Booth Devices
Place a Changing Booth so players can access their lockers and swap outfits during the game.
![Changing Booth Devices](https://dev.epicgames.com/community/api/documentation/image/9cb9b0f4-63cc-402b-80f6-65a03b56ca50?resizing_type=fill&width=1920&height=335)
The **Changing Booth** device lets players access their [Locker](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary) while a game is in progress. This feature is especially useful in [fashion games](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary).
With the Changing Booth device, players can access their locker and change their outfits without going back to the lobby!
To use the Changing Booth, players can press **E** during a game to open the booth, and use it to change their outfit, pickaxe, and so on. Players will not be able to see the island or other players while in the booth.
To find the Changing Booth device, go to the **Creative inventory** and select the **Devices** tab. From there, you can search or browse for the device. For help finding the Changing Booth device, see [Using Devices](https://dev.epicgames.com/documentation/en-us/fortnite/using-devices-in-fortnite).
##  Device Options
The Changing Booth has one device option.
Default values are bold.
Option  |  Value  |  Description
---|---|---
**Enabled at Game Start** |  On, Off |  Determines whether the device is enabled when the game starts.
##  Direct Event Binding
Following are the [direct event binding](https://dev.epicgames.com/documentation/en-us/fortnite-creative/fortnite-creative-glossary#direct-event-binding) options for this device.
###  Functions
Option  |  Description
---|---
Enable When Receiving From |  Enables the device when an event occurs. This allows players to use the changing booth
Disable When Receiving From |  Disables the device when an event occurs. This removes all players from the booth and prevents it from being used.
Remove All Players When Receiving From |  Removes all players from the changing booth when an event occurs.
Remove Instigating Player When Receiving From |  Removes the instigating player when an event occurs.
###  Events
This device has no events.
