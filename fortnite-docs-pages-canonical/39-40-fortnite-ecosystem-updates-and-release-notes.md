## https://dev.epicgames.com/documentation/en-us/fortnite/39-40-fortnite-ecosystem-updates-and-release-notes

# 39.40 Fortnite Ecosystem Updates and Release Notes
Find out what's new with the 39.40 release of Fortnite on February 5, 2026!
![39.40 Fortnite Ecosystem Updates and Release Notes](https://dev.epicgames.com/community/api/documentation/image/21f7c3db-8add-486a-966e-9493c87f4f32?resizing_type=fill&width=1920&height=335)
The Creator Portal now includes a Conversion Rate chart in the In-Island Transactions tab that shows the percentage of active players who made purchases over time. Debug Commands now support Custom Keybinds and new commands, including Change View Mode, Teleport to Aim Point, Change Team, and Change Class. The new Verse fields event type lets you create interactive UI widgets in UMG that respond to user inputs. Check out the Sandy Strip Galleries, new prefabs and more!
##  New Conversion Rate Chart for In-Island Transactions
We’ve added a **Conversion Rate** chart to the **In-Island Transactions** tab within the Monetization page in the Creator Portal. This chart shows the percentage of active players on an island who made an in-island transaction purchase. You can also filter by island, and view trends over time.
To view the conversion rate for your island, navigate to **Monetization** ˃ **In-Island Transactions** in the Creator Portal. The chart appears at the bottom of the tab.
##  Create Interactive UI: Verse Fields Events Now Available in UMG
Create UI widgets that can trigger events in Verse using the new Verse fields event type in UMG. Developers can now react in Verse to events triggered by a user, such as a button click, which unlocks new UI capabilities. Open the **Variables** window in the**UMG Designer** to get started.
Once you add fields of the new event type to a User Widget in the UMG Designer, you can bind them to widget events using View Bindings. Events are reflected in the Verse asset digest for the User Widget, allowing you to await for an event from Verse code to react.
Learn more with **[Using Verse fields event type in a UMG User Widget](https://dev.epicgames.com/community/learning/tutorials/woOK/fortnite-using-verse-fields-event-type-in-a-umg-user-widget)**.
[![](https://dev.epicgames.com/community/api/documentation/image/5cb32092-521a-4183-ac4f-454db9cc9022?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/5cb32092-521a-4183-ac4f-454db9cc9022?resizing_type=fit)
##  Debug Commands
###  Custom Keybinds
You can now bind inputs to a command instead of opening the Debug Command menu to run commonly used commands every time. This includes adding a custom bind for opening the Debug Command menu.
Use the **Enable Debug Input** option to avoid interference from inputs during normal playtesting. The Enable Debug Input option is turned off by default. Setting the option to On activates all custom binds. Custom binds consume input, which means that you won’t be able to perform normal actions using that specific input while custom binds are enabled. To regain full control, use **Disable Debug Input**.
Custom keybinds pair well with Toggle- and Action-type debug commands like **Eliminate Nearby AI** and the new **Teleport to Aim Point**.
###  Additional Commands
We’ve added more commands to help you efficiently debug your projects. Here’s what has been added in 39.40:
  * **Change View Mode** - Used to change the user's view mode to Unlit which can be useful for editing dark environments. View mode will be reset to Lit on Game End.
  * **Teleport to Aim Point** - Teleport to the location the aiming reticule is focused on. If your pawn cannot fit in that location, the command will fail.
  * **Change Team** - Enables the user to instantly change team to any valid team that isn't full. Attempting to switch to an invalid, undefined team fails.
  * **Change Class** - Enables the user to instantly change class to any valid class. Attempting to switch to an invalid, undefined class fails

##  LEGO® Template Updates
###  Added Damage Amplifier Device
The Damage Amplifier Device is now available in LEGO Templates, giving developers more control over high-impact gameplay moments.
###  New Weapon Variations
We have added new weapon variations to:
  * Crossbows
  * Hand Axe
  * Sword

##  Content Browser and Inventory Updates
Expand your environment design with Pacific Break-themed assets from Chapter Seven. This includes prefabs for each Chapter Seven Gallery launched in 39.00. Check out all the new devices and items available this release!
[![Sandy Strip Town Square](https://dev.epicgames.com/community/api/documentation/image/e9c3d5da-e754-482c-bde4-bd6844251ca5?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/e9c3d5da-e754-482c-bde4-bd6844251ca5?resizing_type=fit) Sandy Strip Town Square
###  Galleries
  * Sandy Strip Town Square
  * Sandy Strip Deca Hotel
  * Sandy Strip Wall Gallery
  * Sandy Strip Roof Gallery
  * Sandy Strip Floor Gallery
  * Sandy Strip Outdoor Prop Gallery
  * Sandy Strip Indoor Prop Gallery

[![Ripped Tides Flying Fish Tacos](https://dev.epicgames.com/community/api/documentation/image/1c0f87ae-cf17-4ae3-8ef5-36ec6d04e357?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/1c0f87ae-cf17-4ae3-8ef5-36ec6d04e357?resizing_type=fit) Ripped Tides Flying Fish Tacos
###  Prefabs
  * Ripped Tides Flying Fish Tacos
  * Ripped Tides Skatepark
  * Classified Canyon Check Point
  * Classified Canyon Helipad
  * Wonkeeland Gator Slide
  * Wonkeeland Entrance Gate
  * Wonkeeland Concert Stage
  * Battlewood Boulevard Stores
  * Tiptop Terrace Supply Depot
  * Tiptop Terrace Mile High Retreat
  * Bumpy Bay Chow Durrr
  * Bumpy Bay Neighborhood

####  Drivable Reboot Van Update
The following options and functions have been added to the Drivable Reboot Van Spawner:Options
  * **Is Operable** - Determines if the van can be driven or moved.
  * **Revive Enabled** - Determines if the revive seat is usable. Player must be in **Down But Not Out** to use the revive seat.
  * **Revive Activating Team** - Determines what team can use the revive seat.
  * **Invert Revive Activating Team Selection** - If set to **YES** , the selected Revive Activating Team is the only Team that cannot access revive seat.
  * **Revive Activating Class** - Determines what class can use the revive seat.
  * **Invert Revive Activating Class Selection** - If set to **YES** , the selected Revive Activating Class is the only Class that cannot access revive seat.

**Functions**
  * **Enable Revive** - Enable the revive seat.
  * **Disable Revive** - Disable the revive seat.

The following verse functions have also been added:
  * `EnableRevive<public>():void` - Enable the revive seat.
  * `DisableRevive<public>():void` - Disable the revive seat.
  * `IsEnabledRevive<public>()<decides><transacts>:void` - Succeeds if the revival seat is enabled and fails if it's disabled.

####  Capture Area Device Updates
The following device options have been added:
  * **User-Defined Spline Boundary Shape** - Allows for the creation of custom capture area shapes.
  * **Hide Base Mesh** - A new option to determine if the base mesh is visible.

####  capture_area_device class Updates
The following Verse variable has been added:
  * **CaptureProgress** - A ReadOnly variable used to track current capture progress.

Check out the [Capture Area Devices](https://dev.epicgames.com/documentation/en-us/fortnite/using-capture-area-devices-in-fortnite-creative) and [capture_area_device class](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/capture_area_device) documents for more information.
###  New Weapons
  * Lock On Rifle

###  New Items
  * FlowBerry Mist Grenade

##  Community Bug Fixes
The following fixes address issues you submitted to us on the forums. Thank you for your patience and for reporting these issues!
  * Fixed an issue where dragging a texture from the 'Content Drawer' to an asset did not change the asset's texture.
    * [Forum Report](https://forums.unrealengine.com/t/dragging-a-texture-from-content-browser-no-longer-creates-material/2690887)
  * Fixed an issue where Guards were not responding to patrol paths or navigating the map, depending on the player distance from the spawner.
    * [Forum Report](https://forums.unrealengine.com/t/guards-are-not-responding-to-patrol-paths-or-navigating-the-map-depending-on-player-distance-from-spawner/2571269/20)
  * Fixed the issue where converting Creative Islands to UEFN resulted in a blank project
    * [Forum Report](https://forums.unrealengine.com/t/critical-converting-creative-island-to-uefn-results-in-a-blank-project/2682682)
  * Fixed an issue where players would see a black screen when exiting a UEFN island and returning to the Fortnite lobby.
    * [Forum Report](https://forums.unrealengine.com/t/black-screen-occurs-when-exiting-a-uefn-island-and-returning-to-the-fortnite-lobby/2693180)
  * Fixed an issue where an Interactable component was not able to be interacted with if dynamically spawned within range of the player until they leave said interaction radius and return.
    * [Forum Report](https://forums.unrealengine.com/t/scene-graph-if-a-player-is-standing-within-an-interactable-components-interact-radius-when-it-is-dynamically-spawned-it-will-not-be-interactable-until-the-player-leaves-and-returns-to-the-radius/2678419)

##  Fortnite Ecosystem Updates and Fixes
**New:**
  * You can now showcase additional images of your island on the Island Details page.

**Fixes:**
  * Fixed a bug that caused the Bank Vault device to be unusable by players who joined a game in-progress.

  * Fixed an issue that occasionally prevented the holiday gift box in the Chest and Ammo Gallery from dropping loot.
  * Fixed an issue where Uncommon and Rare rarities of the Suppressed Pistol could fail to drop from the Heist Bag found in the Chest and Ammo Gallery.
  * Fixed an issue that caused the giant present item to spawn without loot.

  * Fixed an issue that caused users to have to resave levels each time they reopened the project if it contained breakable glass assets.

##  UEFN Updates and Fixes
**Fixes:**
  * Fixed an issue with the phone tool’s inability to copy and paste an entity that has its Origin property set through Verse on its Transform Component.
  * Fixed an issue that caused the player to lose inputs after pushing changes to the client in UEFN.

###  Editor
**Fixes:**
  * Fixed an issue where converting an older island to a UEFN project would create an empty project.
  * Fixed an issue where some drop operations in the level editor viewport were shown as unsupported (for example, applying a material to an actor).

###  Environments and Landscapes
**New:**
  * Clamps the WaterZone render target resolution to improve performance on low-end devices with maps utilizing the Ocean, Lake, and River actors.
  * Added support for all known texture format types.

**Fixes:**
  * Fixed crashes related to the landscape spline modulation texture rasterization (for example, out-of-bounds sampling, wrong channel count, and more).

###  In-Game UI
**New:**
  * Added the option Display (monitor) to video settings.
  * Added new **Radial Gauge** material to the Fortnite UI folder. See [UI Materials Collection](https://dev.epicgames.com/documentation/en-us/fortnite/ui-materials-collection-in-fortnite) documentation to learn more.

##  Scene Graph
New:
  * Entities render as **selected** when their prefab root is selected.
  * Using the content browser to create an EntityPrefab, adds a `transform_component` to the asset.
  * Added the **Entity Prefab** entry name to the content browser context menu.
  * Changed Entity Prefab asset default name to **EntityPrefab**.

Fixes:
  * Fixed an issue in the prefab editor that didn't retain edits on parent entity nodes that included a `replication_component`.
  * Fixed an issue that prevented entities created with drag and drop from showing as selected in the Prefab Editor outliner.
  * Fixed an issue in the Prefab Editor where reparented child entities would vanish during undo and redo actions.
  * Fixed an issue that deleted entities after undoing a reparenting operation.
