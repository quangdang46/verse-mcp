## https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite



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
  4. 33.20 Fortnite Ecosystem Updates and Release Notes


# 33.20 Fortnite Ecosystem Updates and Release Notes
33.20 Fortnite Ecosystem Updates and Release Notes in Creative, Unreal Editor for Fortnite, and Verse 
![33.20 Fortnite Ecosystem Updates and Release Notes](https://dev.epicgames.com/community/api/documentation/image/e3a34309-1673-475d-af86-f22ea2604c03?resizing_type=fill&width=1920&height=335)
On this page
Wishing our community an amazing year ahead!
With the v33.20 update, you can now use the Air and Water Sprites from CH6S1 Battle Royale in the Wildlife Spawner device. The new Earth Sprite device lets players trade weapons for a Legendary weapon. Plus, new Hunting rifles have arrived.
Here's to another year of amazing creations – happy 2025!
##  Device Updates: Battle Royale Sprites 
###  Air And Water Sprites Available in Wildlife Spawner 
The Air and Water Sprites from CH6S1 Battle Royale are now available in the Wildlife Spawner device. Players can collect these sprites and use their unique abilities.
###  New Earth Sprite Device 
Creators can now use the Earth Sprite from CH6S1 Battle Royale in their islands by using the **Earth Sprite** device. Players can bring a weapon to the Earth Sprite, and it returns a Legendary weapon to the player. Creators can customize the loot pool by dropping weapons on the device, modify how the Earth Sprites get full, determine the message text, and more.
##  New Weapons 
  * Mythic Hunting Rifle
  * Rare Hunting Rifle
  * Uncommon Hunting Rifle


##  Community Bug Fixes 
The following list of fixes are from issues that you submitted to us on the forums. Thank you for your patience and for reporting these issues!
  * Fixed increased hitch rate by 700-900% on certain maps
    * [**Forum Issue Report**](https://forums.unrealengine.com/t/critical-hitch-rate-increased-by-700-900-on-certain-maps/2165630/9)
  * Fixed issue where cutting and pasting certain assets while in a Live Edit session causes the pasted assets to lose texture overrides
    * [**Forum Issue Report**](https://forums.unrealengine.com/t/texture-bug-after-object-placement/2173613/8)
  * Fixed Perforce being unusable in UEFN
    * [**Forum Issue Report**](https://forums.unrealengine.com/t/perforce-has-become-unusable-in-uefn/2195577)


##  Fortnite Ecosystem Updates and Fixes 
**New** :
  * Added "Holiday Presents!" to Creative.
  * Updated doc strings for `FindCreativeObjectsWithTag` to clarify when an empty result can be expected.
  * Added `FindCreativeObjectsWithTag` extension method for `npc_behavior`.


**Fixes** :
  * Fixed an issue with the Demon Fire Mask's homing reticle not showing on creatures and wildlife.
  * Fixed issues with collision for gallery objects.
  * Fixed an issue where the island download progress only used the first 20% of the bar.
  * Fixed a crash that occurred when manipulating AI Patrol Path devices.
  * Updated the pack location for SwissKale to sit closer to the character.


###  Scene Graph 
**New** :
  * light_components now use centimeters like other SceneGraph components.


**Fixes** :
  * Circular dependencies are no longer allowed when picking entities.


###  Scene Graph Upgrade Notes 
  * If you created and saved experimental light_components in 33.00 or 33.10, they need to have their values changed to centimeters starting in 33.20.


###  Editor 
**New** :
  * Added a Default button to the Scalability Menu in the Editor that resets the scalability to the initial setting in the device profile.


**Fixes** :
  * Fixed an issue where the Editor loading caused a potential crash or unwanted side-effects on undo.
  * Fixed padding around the Experimental Feature warning.
  * Fixed an issue with NPCs showing up in custom UI player info lists.
  * Fixed an issue where creating an anchored Note would unset the Editor’s selection mode.
  * The following are fixes made to **Live Edit** :
    * Fixed an issue where the transform of an actor was sometimes lost (for example, changing the rotation and losing the scale).
    * Fixed an issue with new traps not being added in UEFN.
    * Fixed an issue where cutting and pasting wall or floor pieces left the old actor behind in UEFN.
    * Fixed some states failing to sync to UEFN when placing prop variants.


###  Unreal Revision Control 
  * Added a warning for not having a user signed into an AutomationException for Xbox devices.


##  Verse Updates and Fixes 
**New** :
  * Added more support for type aliases at module scope in the form `id := expr`: this can now be used to create aliases for classes, interfaces, and other nominal types.
  * Improved the reliability of adding and removing widgets in Verse.


**Fixes** :
  * Fixed a bug in `ApplyWorldRotation` for Verse rotations.


##  Known Issues 
Chapter 5 Season 1 Core Weapons have experienced some animation regressions while using the First Person Device. This will be improved in future updates throughout January.
  * [ verse](https://dev.epicgames.com/community/search?query=verse)
  * [ release notes](https://dev.epicgames.com/community/search?query=release%20notes)
  * [ uefn](https://dev.epicgames.com/community/search?query=uefn)
  * [ bugs](https://dev.epicgames.com/community/search?query=bugs)
  * [ unreal editor for fortnite](https://dev.epicgames.com/community/search?query=unreal%20editor%20for%20fortnite)
  * [ fixes](https://dev.epicgames.com/community/search?query=fixes)
  * [ creative](https://dev.epicgames.com/community/search?query=creative)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [ Device Updates: Battle Royale Sprites ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#device-updates-battle-royale-sprites)
  * [ Air And Water Sprites Available in Wildlife Spawner ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#air-and-water-sprites-available-in-wildlife-spawner)
  * [ New Earth Sprite Device ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#new-earth-sprite-device)
  * [ New Weapons ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#new-weapons)
  * [ Community Bug Fixes ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#community-bug-fixes)
  * [ Fortnite Ecosystem Updates and Fixes ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#fortnite-ecosystem-updates-and-fixes)
  * [ Scene Graph ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#scene-graph)
  * [ Scene Graph Upgrade Notes ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#scene-graph-upgrade-notes)
  * [ Editor ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#editor)
  * [ Unreal Revision Control ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#unreal-revision-control)
  * [ Verse Updates and Fixes ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#verse-updates-and-fixes)
  * [ Known Issues ](https://dev.epicgames.com/documentation/en-us/fortnite/33-20-fortnite-ecosystem-updates-and-release-notes-in-creative-and-unreal-editor-for-fortnite#known-issues)






---
