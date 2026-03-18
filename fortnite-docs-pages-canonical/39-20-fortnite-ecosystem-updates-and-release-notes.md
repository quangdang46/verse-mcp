## https://dev.epicgames.com/documentation/en-us/fortnite/39-20-fortnite-ecosystem-updates-and-release-notes

# 39.20 Fortnite Ecosystem Updates and Release Notes
Find out what's new with the 39.20 release of Fortnite on January 09, 2026!
![39.20 Fortnite Ecosystem Updates and Release Notes](https://dev.epicgames.com/community/api/documentation/image/c7b1daa9-c42d-424a-aa36-0a096d50bd91?resizing_type=fill&width=1920&height=335)
Publishing for in-island transactions goes live today! We’ve also added UI improvements that make custom buttons and HUD elements easier to build across platforms. Plus, check out the new set of SUS Studios prefabs and Galleries, along with new and updated weapons.
##  In-Island Transactions Publishing Update
Publishing for in-island transactions will go live later today, and players will be able to purchase items in your islands using V-Bucks.
Take a moment to review the latest [In-Island Transactions documentation](https://dev.epicgames.com/documentation/en-us/fortnite/in-island-transactions-in-fortnite). We updated the [FAQs](https://dev.epicgames.com/documentation/en-us/fortnite/in-island-transactions-overview-in-fortnite#frequently-asked-questions) and added the new [Guidelines for In-Island Monetization](https://dev.epicgames.com/documentation/en-us/fortnite/guidelines-for-in-island-monetization-in-fortnite) page.
##  Unreal Revision Control Migration
Over the coming weeks, all **Unreal Revision Control** (URC) projects created before the 36.00 release last June will be migrated to the latest version (URC2). We made this change to lay the groundwork for upcoming features to be available to all projects.
You should not experience any disruptions working on these projects, but please reach out [in the forums](https://forums.unrealengine.com/) if you do.
##  Connect with Players Through Fortnite Communities
[Fortnite Communities](https://communities.epicgames.com/) is now live and provides another way to connect with players and grow your audience. Create your community, set up channels, and publish posts with island updates, event information, feedback requests, and more.
[![](https://dev.epicgames.com/community/api/documentation/image/4a124a97-44be-415d-833e-822f47c28cd4?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4a124a97-44be-415d-833e-822f47c28cd4?resizing_type=fit)
For additional information on setting up your community, check out the [Fortnite Communities documentation](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-communities-in-fortnite).
##  UI: Events and Materials
###  Verse Button and UMG Highlight Events
The **Verse button class** has two new events:
  * **HighlightEvent**
  * **UnhighlightEvent**

You can use these events to create custom button designs in Verse code.
An additional two events, **On Highlight** and **On Unhighlight** , were added to the buttons available in UMG, which you can use to create bindings. Highlight events fire when a long press is detected or released on touchscreen devices. Otherwise, they behave similarly to **Hover** events.
We strongly encourage using the new highlight events instead of the existing**On Hover** and **On Unhover** events, as they provide better support for touchscreen platforms.
###  Gauge Material
A new material, **M_UI_RadialGauge** , was added to the UI materials available in UEFN under the **Fortnite > UI > Materials** folder. You can use this versatile material to make a UI for a variety of gauges and speedometers.
The radial gauge material pairs well with the recent updates to the [HUD Controller](https://dev.epicgames.com/documentation/en-us/fortnite/using-hud-controller-devices-in-fortnite-creative) device for hiding the default vehicle HUD and the new Verse API to get vehicle data, providing the means for developers to make their own custom vehicle HUD UIs.
[![M_UI_RadialGauge in UEFN](https://dev.epicgames.com/community/api/documentation/image/ac589df7-3eb3-4ff1-a123-85e57b8203ea?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/ac589df7-3eb3-4ff1-a123-85e57b8203ea?resizing_type=fit)
Each [meter material](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-glossary#meter-materials) and customizable texture have GIFs that illustrate how the material reacts when you change a parameter in the material instance, along with information on using material parameters with View Bindings in UMG.
[![](https://dev.epicgames.com/community/api/documentation/image/4f3b164c-d5d3-400e-b146-ce9515ffb0f7?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4f3b164c-d5d3-400e-b146-ce9515ffb0f7?resizing_type=fit)
For more information, see the [UI Materials Collection](https://dev.epicgames.com/documentation/en-us/fortnite/ui-materials-collection-in-fortnite) and [UI Texture Material Collection](https://dev.epicgames.com/documentation/en-us/fortnite/ui-texture-material-collection-in-fortnite) pages.
##  Debug Commands
To help you efficiently debug your projects, we added more debug commands to the Beta Debug Command menu. You can now enable or disable:
  * **Infinite Consumables**
  * **Infinite World Resources**
  * **Infinite Durability**
  * **Infinite Charges**

##  Content Browser and Inventory Updates
Check out all the new devices and items available this release!
###  Driveable Reboot Van
The **Drivable Reboot Van Spawner** device is now available in Creative and UEFN. Provide players with opportunities to revive teammates on the go with this vehicle-based version of the [Reboot Van](https://dev.epicgames.com/documentation/en-us/fortnite/using-reboot-van-spawner-devices-in-fortnite-creative) device, featuring the same user options.
The drivable variant includes a fifth seat for players to revive teammates who are down but not out, as well as the Verse API.
[![](https://dev.epicgames.com/community/api/documentation/image/269fbcca-f4aa-4587-92db-38060a212d5d?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/269fbcca-f4aa-4587-92db-38060a212d5d?resizing_type=fit)
###  Device Updates and Fixes
**Fixes:**
  * Fixed the original Player Reference device no longer passing validation.
  * Fixed an issue with Reboot Van's Enable and Class/Team User Option not being respected in UEFN.

###  New Weapons
  * **Flex SMG**
  * **Tactical Assault Rifle**

This launch also includes updates to existing weapons:
  * **Sovereign Shotgun:** New non-modular version.
  * **Drum Gun:** New non-modular version.
  * **Dual Pistols:** Common rarity added.
  * **Six Shooter:** Legendary rarity added.
  * **Slone’s Striker Burst Rifle:** Mythic rarity added.

###  New Items
  * **Stealth Splash**

###  New Prefabs & Galleries
  * **SUS Studios Warehouse Prefab**
  * **SUS Studios HQ Prefab**
  * **SUS Studios Floor Gallery**
  * **SUS Studios Wall Gallery**
  * **SUS Studios Roof Gallery**
  * **SUS Studios Prop Gallery**
  * **SUS Studios Tunnel and Street Gallery**

##  Epic’s Picks - What’s New
We’ve updated the [Epic’s Picks documentation](https://dev.epicgames.com/documentation/en-us/fortnite/epics-picks-in-fortnite) with the latest guidelines, including Q1 prioritized genres and themes. Moving forward, refer to this documentation for the most current information and to ensure your island meets all criteria for potential feature consideration.
##  Community Bug Fixes
The following fixes are for issues that developers submitted to us on the forums.
  * Fixed an issue where not all players were appearing on the scoreboard.
    * [Forum Report](https://forums.unrealengine.com/t/scoreboard-broken-after-v38-00/2670849)
  * Fixed an issue where unknown assets were appearing in memory calculations and using a large amount of memory.
    * [Forum Report](https://forums.unrealengine.com/t/unknown-fortnite-asset-taking-25k-memory/2671505)
  * Fixed an issue where players were losing items when leaving a game, even with the Save Point device turned on.
    * [Forum Report](https://forums.unrealengine.com/t/players-lose-all-items-when-leaving-a-game-even-if-the-save-point-device-is-turned-on/2676582/11)
  * Fixed an issue where an exploded Guided Missile was not behaving as expected on player respawn.
    * [Forum Report](https://forums.unrealengine.com/t/guided-missile-issue-in-uefn-creative-maps/2449792)

##  Fortnite Ecosystem Updates and Fixes
**New:**
  * Added an option to filter the cosmetic loadout component to only replicate what you use.

**Fixes:**
  * Updated the tall corrugated shutter in the back of the Sus Studios Wall Gallery to open and close with the Lock device.
  * Made a minor optimization for network replication usage.
  * Addressed additional instances of missing or incorrect season Creative tags on weapons and items.
  * Added support for automated conversion bindings from the Quick Binding menu in the View Model.
  * Renamed creative island Join in Progress option from "Watch Only" to "Spectate Only".
  * Made a minor optimization for speed of loading into gameplay for mobile devices.
  * Fixed players holding Mic pickaxes incorrectly after landing from deployment.
  * Fixed a bug with the Immersive Edit Mode option in Creative, where the setting would only save if it was changed while the phone tool was equipped.
  * Fixed an issue where SFX and VFX were not reactivated when their network relevancy changed. This could typically occur at the start of a match.

##  UEFN Updates and Fixes
**New:**
  * Added more Hermes_Gravel and Hermes_Scree to the Fortnite > Environment > Foliage folder in the Content Browser.

**Fixes:**
  * Fixed a crash where unsubscribing potentially unloaded `UStructs` in CVD Sessions Manager.
  * Make the CVD Scene Outliner update during playback by default.
  * Fixed a crash that could occur when using Modeling Mode's Self-Union, Mesh Cut, or Mesh Boolean tools on meshes with invalid material IDs.
  * Fixed landscape spline modulation texture rasterization crashes (out of bounds sampling, wrong channel count).
  * Added support for all known texture format types to landscape spline modulation texture rasterization.
  * Fixed a crash and an incorrect "overridden" notification for some edge cases.

##  Scene Graph Updates and Fixes
**New:**
  * Added an option to create a prefab subclass from the Content Drawer context menu.
  * Enabled realtime updates in Prefab Editor by default.
  * Introduced a new drag-and-drop workflow to the editor viewports and outliners. Prefab Editor drag and drop now supports previews.
  * Added a Group Under New Entity menu item to the context menu of the Entity Prefab Editor.
  * Enabled creating new prefab subclass from existing prefab within the editor.

**Fixes:**
  * Improved Prefab Editor focus behavior, added cvars to control it, focus on root entity when first opening prefab editor, refresh viewport on component addition or removal, and updated the gizmo on transform manipulations in Details panel.
  * Updated the property matrix to show entity components when `PropertyEditor.AllowPropertyMatrixForNonActors` is true. Most items are still read-only.
  * Updated the editor to preserve component type selection when changing entity selection.
  * Updated the thumbnail in the Details panel of the editor to show the common asset when selecting multiple entities that have the same asset component (that is, a mesh_component that is the same asset).
  * Updated the Prefab Editor to select the parent prefab when clicking on an entity.
  * Fixed the prefab in-place editor mode to reparent pasted entities to the current entity.
  * Fixed an issue in which duplicated entity prefabs renamed their derived children.
  * Fixed viewport positioning ignoring entities for collision.
  * Fixed an undo and redo issue for prefab entities after closing Prefab Editor.
  * Fixed Prefab Editor memory not being freed after closing.

##  Verse Updates and fixes
**Fixes:**
  * Fixed `<override>` instantiations that experienced data loss after launching a session/publishing content in UEFN.
  * Fixed the source of an `ensure()` that was frequently being reported in user projects after loading a previously-saved Verse creative device script instance that failed to load as part of an island.

##  Unreal Revision Control (URC) Updates & Fixes
**Fixes:**
  * Fixed an issue causing especially long times connecting to URC for the first time on projects with thousands of snapshots.
  * Fixed an issue that caused long “reverting files in revision control” dialogs after deleting assets from the Content Browser, and occasional periods of editor unresponsiveness after revision control actions such as reverting or syncing a large snapshot.
  * Fixed an issue preventing graceful recovery / reconnection in cases where a user deleted their .urc folder.
