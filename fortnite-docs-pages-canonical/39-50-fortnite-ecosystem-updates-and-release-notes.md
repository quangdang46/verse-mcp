## https://dev.epicgames.com/documentation/en-us/fortnite/39-50-fortnite-ecosystem-updates-and-release-notes

# 39.50 Fortnite Ecosystem Updates and Release Notes
Find out what's new with the 39.50 release of Fortnite on February 19, 2026!
![39.50 Fortnite Ecosystem Updates and Release Notes](https://dev.epicgames.com/community/api/documentation/image/278b5bce-1473-4659-8a65-17610c3b22b6?resizing_type=fill&width=1920&height=335)
New community management tools are now available for [Fortnite Communities](https://communities.epicgames.com/). These features allow community owners to assign team roles and issue temporary or permanent bans. This release also expands animation functionality across the Player Reference, Character, and Chair devices, and adds new Fore Fields prefabs and Galleries to the Content Browser.
##  Fortnite Communities: New Community Management Tools Now Available
We've added new features to help you manage your [Fortnite Communities](https://communities.epicgames.com/):
  * **Assign Team Roles:** Community owners can now assign roles to trusted team members to participate as moderators and help manage their communities. Navigate to **Community Settings > Team** to get started.
[![](https://dev.epicgames.com/community/api/documentation/image/956c102e-facd-4eb3-b25c-3e4f8d314bda?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/956c102e-facd-4eb3-b25c-3e4f8d314bda?resizing_type=fit)

  * **Ban Users:** You can also now issue temporary or permanent bans for users. To ban a user, click the ellipsis menu next to their post and select **Ban**. Once banned, users cannot post or reply in your community.
[![](https://dev.epicgames.com/community/api/documentation/image/8986e73b-42cc-4536-b381-1c3d565f9af9?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/8986e73b-42cc-4536-b381-1c3d565f9af9?resizing_type=fit)

Check out the full [documentation](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-communities-in-fortnite) to learn more.
##  Verse UI Input Actions Experimental
We have exposed a set of input actions and mapping contexts for use in Verse UI mode in an experimental release. Actions such as Craft, Drop and Sort can now be assigned to a verse button's **TriggeringInputAction** property. Doing so will cause the **OnClick** event of the button to be called when the associated key is pressed. The associated mapping context must be activated from Verse Input API.
The actions are only responsive when UI mode is active (such as when the Verse UI takes input). The available input actions and mappings are currently under [Control Input UI](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput).
Note that the introduction of this **UI** module may cause name ambiguity errors in existing projects. If you observe Verse errors due to having a UI module in your project, please rename the module and update the **using** directive in your verse code to resolve the ambiguity.
##  Content Browser and Inventory Updates
Check out all the new devices and items available this release!
###  Animation Updates to Devices
The Player Reference device, Character device, and Chair device have received new functionalities to improve how you can work with animations on characters.
  * **Player Reference Device:** Allows Sequencer to animate a mannequin of the player tied to the player reference device when it is used in a cinematic sequence.
  * **Character Device:** Similarly allows Sequencer to animate the mannequin portion of the device.
  * **Chair Device:** Exposes the**Custom Sit Animation** option for playing an animation directly on the seated character.

###  New Prefabs & Galleries
  * Fore Fields Wall Gallery
  * Fore Fields Floor Gallery
  * Fore Fields Roof Gallery
  * Fore Fields Prop Gallery
  * Fore Fields Foundation Gallery
  * Fore Fields Golf Range
  * Fore Fields Club House

##  Community Bug Fixes
The following fixes address issues you submitted to us on the EDC Forums. Thank you for your patience and for reporting these issues!
  * Fixed an issue where some placed chests didn’t spawn.
    * [Forum Report](https://forums.unrealengine.com/t/chests-spawn-randomly-since-39-00/2681994)
  * Fixed an issue where some islands were blocked from publishing because of experimental features, even though the features weren’t presented in the island.
    * [Forum Report](https://forums.unrealengine.com/t/blocked-from-publishing-for-island-transactions-experimental-feature/2680293)
  * Fixed an issue where players were able to fire the Ballistic Shield weapon through walls to damage enemies.
    * [Forum Report](https://forums.unrealengine.com/t/why-can-a-ballistic-shield-be-used-to-shoot-through-walls-doors-or-barriers/2625092)
  * Fixed an issue where some Shipping Container assets in UEFN had unintended textures applied to them.
    * [Forum Report](https://forums.unrealengine.com/t/shipping-container-material-bugged/2164201)
  * Fixed a visual error on the Suppressed Sniper Rifle.
    * [Forum Report](https://forums.unrealengine.com/t/suppressed-sniper-rifle-little-visual-error/2672143)

##  Fortnite Ecosystem Updates and Fixes
**New:**
  * Added the new Mythic rarity Empowered Cupid's Crossbow.

**Fixes:**
  * Fixed an issue where players under the effects of Nitro Splash could no longer break through wooden structures.
  * Fixed an issue where the Nitro Barrel Spawner device couldn't be placed with the phone tool while editing an island.

  * Fixed the Creative tagging on the Modular Monarch Pistol. It was incorrectly tagged as a light ammo weapon when it actually uses medium ammo.
  * Fixed the Creative tagging on the Super Launch Pad. It was incorrectly tagged as Legendary when it's actually Epic rarity.
  * Fixed an issue where if the Lawless Slap Cannon was dropped directly from the Creative Inventory menu while editing your island, it couldn't be fired.
  * Fixed Legendary Slurp Juice not being usable while falling.

##  UEFN Updates and Fixes
**Fixes:**
  * Fixed a problem that caused developers to lose certain inputs when publishing and launching a play session in UEFN.
  * Fixed a crash that occurred when recompiling verse code (or running validation).
  * Fixed an issue where certain SetFocus() calls did not function correctly.
  * Fixed an issue where the Creative quick bar disappeared when connecting a controller on mobile devices.
  * Fixed an issue where click feedback caused Verse buttons to visibly jitter.
  * Fixed an issue where consumable quantities started at 100 when Use Latest HUD was enabled in the Island Settings.

  * Fixed edge case in which drag and drop in the viewport would place two duplicate objects.

  * Fixed a cook crash with Landscapes storing an invalid (empty named Target Layer). It now ensures invalid target layers are properly removed during the load-and-cook process.

##  Scene Graph
**Fixes:**
  * Fixed a bug that could crash the editor when a prefab was saved. If the prefab contained a nested prefab that was also open for edit, the editor would crash.

##  Unreal Revision Control Updates and Fixes
**Fixes:**
  * Fixed an editor crash that could occur when trying to check in changes in rare cases where a file being deleted had an invalid file history.
