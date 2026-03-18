## https://dev.epicgames.com/documentation/en-us/fortnite/39-30-fortnite-ecosystem-updates-and-release-notes

# 39.30 Fortnite Ecosystem Updates and Release Notes
Find out what's new with the 39.30 release of Fortnite on January 22, 2026!
![39.30 Fortnite Ecosystem Updates and Release Notes](https://dev.epicgames.com/community/api/documentation/image/818e6c66-8165-4de6-8bca-ccb3a06a971b?resizing_type=fill&width=1920&height=335)
Fortnite Tools Mode adds new optimization and physics tools to help you quickly identify performance issues and streamline your island builds.
We’ve also updated the User Interfaces feature template with new examples for building custom Verse buttons, creating UI driven by Verse data, and designing custom vehicle HUDs in UEFN.
Plus, check out the new Brutal Bastion prefabs and Galleries.
[![](https://dev.epicgames.com/community/api/documentation/image/7fd716c9-35d1-4b77-8f54-57b700f8b53f?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/7fd716c9-35d1-4b77-8f54-57b700f8b53f?resizing_type=fit)
##  fort_item_pickup_component Deprecated
We have deprecated the `fort_item_pickup_component` in 39.30. This component is part of the experimental release of **Custom Inventory and Items**.
If you are using the `fort_item_pickup_component` in an experimental project, you can replace it with an `interactable_component` subclass. For more information, see [Create an Item Pickup Interactable Component](https://dev.epicgames.com/documentation/en-us/fortnite/create-an-item-pickup-interactable-component-in-fortnite).
In the future, the functionality currently provided by `fort_item_pickup_component` will be exposed as separate modular Scene Graph components.
##  Debug Commands
We’ve added more commands to help you efficiently debug your projects. Here’s what has been added in 39.30:
  * **Toggle FN HUD Visibility** - Toggle the visibility of all Fortnite HUD elements. This does not include custom UI.
  * **Change Time Of Day** - Change the time of day.

##  User Interfaces Feature Template Updates
The UI Features Example template has been updated to demonstrate new UI features for UEFN. Try out the updated template to find examples for how to make custom Verse buttons, custom UI driven by Verse data, and custom vehicle HUDs.
[![](https://dev.epicgames.com/community/api/documentation/image/353c164a-b133-461a-850d-655daaf24965?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/353c164a-b133-461a-850d-655daaf24965?resizing_type=fit)
The above pictures show examples for using Verse fields to make UI widgets using UMG that are be driven by custom gameplay data (left) and how to build custom buttons in Verse (right).
[![](https://dev.epicgames.com/community/api/documentation/image/566ae493-3a61-447d-8655-55ad47ddab58?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/566ae493-3a61-447d-8655-55ad47ddab58?resizing_type=fit)
The above picture presents an example for using custom Verse buttons, Verse fields, and the In-Island Transactions API to make custom menus.
[![](https://dev.epicgames.com/community/api/documentation/image/f71c07d5-2224-45b4-bfae-8d2c1412b80a?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f71c07d5-2224-45b4-bfae-8d2c1412b80a?resizing_type=fit)
[![](https://dev.epicgames.com/community/api/documentation/image/376edc55-8567-4a38-813b-315fa8609fe0?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/376edc55-8567-4a38-813b-315fa8609fe0?resizing_type=fit)
The above pictures show a new UI material that can be used to create gauges and speedometers (left) and an example of using it with recently added Verse API for getting vehicle data such as speed and boost to create custom vehicle HUD widgets.
##  Ch7 BR HUD Update
The **Ch7 BR HUD** can now be enabled using the **Use Latest HUD** island option. The island setting will be available for a limited number of releases so custom HUD elements should be adapted during this time if there are any conflicts.
[![](https://dev.epicgames.com/community/api/documentation/image/9d99436e-6e9a-463e-9fb2-3a2d22e82386?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/9d99436e-6e9a-463e-9fb2-3a2d22e82386?resizing_type=fit)
##  Content Browser and Inventory Updates
Check out all the new items available this release!
###  New Prefabs & Galleries
[![](https://dev.epicgames.com/community/api/documentation/image/dfdbf94f-5276-4db2-8727-34a44c9dbcb1?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/dfdbf94f-5276-4db2-8727-34a44c9dbcb1?resizing_type=fit)
  * Brutal Bastion Base
  * Brutal Bastion Watch Tower
  * Brutal Bastion Wall Gallery
  * Brutal Bastion Floor and Stairs Gallery
  * Brutal Bastion Roof Gallery
  * Brutal Bastion Prop Gallery
  * Brutal Bastion Nature Gallery

[![](https://dev.epicgames.com/community/api/documentation/image/37c447b2-c058-491d-9806-0dc3942f3939?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/37c447b2-c058-491d-9806-0dc3942f3939?resizing_type=fit)
##  Discover Island Details Page Updates
The **About** section on the Island Details page now indicates whether **in-island transactions** are available in your island, and the **Favorite** button displays the island’s favorite count.
##  Fortnite Tools Mode
**Fortnite Tools Mode** has new tools to help you build your island:
  * **Add Physics** - Quickly add, remove, and search for objects with or without physics properties.
  * **Optimize Textures** - This tool runs optimization checks on textures in your project and provides suggestions for how to optimize them for lower-end platforms such as mobile.
  * **Optimize Static Mesh** - Runs optimization checks on static meshes in your project and provides suggestions to optimize them for lower-end platforms such as mobile.

The new optimization tools (Textures and Static Mesh) can help you quickly find and solve optimization issues that can create bloat in your project and potentially cause quality issues for players. See the [Fortnite Tools Mode](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-tools-mode-in-fortnite) document for examples and more information on the new tools.
##  Release Notes
##  Community Bug Fixes
The following fixes are from issues that you submitted on the forums. Thank you for your patience and for reporting these issues!
  * Fixed an issue where the Save device occasionally failed to load.
    * [Forum Report](https://forums.unrealengine.com/t/save-device-sometimes-doesnt-load-inventory/2650414)
  * Fixed an issue where the setting **Persistence Behavior: Edit Session** was not behaving as expected when set to **Import Live Data**.
    * [Forum Report](https://forums.unrealengine.com/t/edit-session-persistence-not-loading/2358846)
  * Fixed an issue where some items would not upgrade to the correct rarity.
    * [Forum Report](https://forums.unrealengine.com/t/upgrading-some-items-does-not-work-properly-hasnt-been-fixed-despite-the-bug-being-marked-as-fixed/2661859)
  * Fixed an issue where round-based games stopped granting XP upon join-in-progress.
    * [Forum Report](https://forums.unrealengine.com/t/round-based-games-stop-granting-xp-when-joining-in-progress/2333124)
  * Fixed an issue where players would respawn with an incorrect camera angle if eliminated while using the Wingsuit.
    * [Forum Report](https://forums.unrealengine.com/t/camera-glitch-when-eliminated-while-gliding-with-wingsuit-item/2680301)

##  Fortnite Ecosystem Updates and Fixes
**New:**
  * You can find and place the Epic and Legendary Heavy Shotguns from Chapter 1 Season 3 in the content browser again.

**Fixes:**
  * The following weapons now properly work with the Midas Flopper:
    * Heavy Assault Rifle
    * Hunting Rifle
    * Pump Shotgun
    * Creative Fishing Rod
    * Guided Missile
    * Infantry Rifle
    * Combat Shotgun
  * Fixed an issue where the holiday gift box in the Chest and Ammo Gallery wouldn't always drop loot.
  * Fixed issue where Infinity Blade's Slam Attack ignored the Friendly Fire island setting.
  * Fixed issue where Precision Air Strike ignored the Friendly Fire island setting.
  * Fixed the Arc-Lightning Gun pivot point when it is in pickup state.
  * Fixed a brief flash that would appear on screen when users were browsing in-island transaction offers.
  * Fixed an issue on mobile devices where the secondary fire button did not appear when players used the Signal Remote Manager.
  * Fixed an issue where crafting resources failed to display permanently on the HUD.
  * Fixed an issue where the Sport Bike default was sometimes not being hidden when using the HUD Controller device.
  * Fixed an issue where player names were not visible after the player left then rejoined a session.
  * Fixed an issue where Island Settings were sometimes not saved.
  * Fixed an issue that caused duplicate or overwritten items on the hotbar when players swapped the items between slots.

##  UEFN Updates and Fixes
**New:**
  * Shortened, less verbose log files will now be generated from input modes.
  * Added support for all known texture format types.

**Fixes:**
  * Fixed irrelevant warnings that occurred in the Output Log when certain templates were selected.
  * Fixed an issue where invalid Verse properties sent to the server over LiveEdit would sometimes crash the server.
  * Fixed an issue with incorrect audio timecode for the Stereo Video Ingest device.
  * Fixed an issue with landscape texture rasterization crashes (out-of-bounds sampling, wrong channel count).

##  Scene Graph
**New:**
  * The `fort_item_pickup_component` is becoming Epic internal only. This will prevent its use in projects where it would break any items that used it.

##  Verse Updates and fixes
**Fixes:**
  * Fixed a **Push Verse Changes** regression in UEFN that caused a failure to instantiate new nodes on the edit server after new fields that had not been seen yet by the content pipeline were generated
