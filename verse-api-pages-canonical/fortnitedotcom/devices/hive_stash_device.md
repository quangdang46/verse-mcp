## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device



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
  4. hive_stash_device class


# hive_stash_device class
Learn technical details about the hive_stash_device class. 
On this page
A strange organic object that may have something or someone inside of it.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description  
---|---  
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.  
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.  
## Exposed Interfaces
This class exposes the following interfaces:
Name | Description  
---|---  
[`enableable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/enableable) |  Implemented by classes whose instances can be enabled and disabled.  
[`healthful`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healthful) |  Implemented by Fortnite objects that have health state and can be eliminated.  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`CanBeDamaged` | `?logic` |  Whether the hive stash can be damaged.
  * `true` by default.

  
`CanDropItems` | `?logic` |  Whether the hive stash drops items when opened. This can happen in addition to an `agent` being rescued.
  * `true` by default.

  
`HiveStashStyle` | `?hive_stash_style` |  Determines what players can see inside the hive stash, as well as its interaction text. This does not affect what happens when it opens.
  * `Empty` shows nothing and says `Break Open`.
  * `Loot` shows a chest and says `Break Open`.
  * `Trapped` shows a floating character and says `Rescue`.

  
`InteractTextOverride` | `?message` |  If set, this overrides the text shown when interacting while the hive stash is closed.  
`OpenEvent` | `listenable(payload)` |  Triggers whenever the hive stash is opened, returning the instigating `agent` if applicable.  
`RescueAnimationEndEvent` | `listenable(payload)` |  Triggers when the rescue animation ends.
  * Returns the rescued `agent`.

  
`RescueEvent` | `listenable(payload)` |  Triggers whenever an `agent` is rescued, either by the hive stash opening with a linked spawner, or by calling _RescueAgent_ while the hive stash is closed.
  * Returns the rescued `agent`.

  
`ShouldKeepLinkOnReset` | `?logic` |  Whether to keep or clear a spawner link when _Reset_ is called. Links can always be cleared with _ClearSpawnerLink_.
  * `true` by default.

  
`ShouldPlayRescuedAnimation` | `?logic` |  Whether an `agent` spawned from registered data or teleported by _FreeAgent_ plays a short animation to get them off the hive stash.
  * The animation may not work properly on skeletons other than the average player skeleton.
  * `true` by default.

  
### Functions
Function Name | Description  
---|---  
[`ClearSpawnerLink`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/clearspawnerlink) |  Clear the link between the hive stash and a spawner device.  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/disable) |  Disable the device, preventing players from interacting with it. Changes to health and state, such as those caused by damage, can still be applied.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/enable) |  Enable the device, allowing players to interact with it.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetHealth`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/gethealth) |  Get the device's current health.  
[`GetMaxHealth`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/getmaxhealth) |  Get the device's current maximum health.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`HasLinkedSpawner`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/haslinkedspawner) |  Succeeds if the device currently has a linked spawner, fails otherwise.  
[`IsEnabled`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/isenabled) |  Succeeds if the device is enabled, fails if it's disabled.  
[`IsOpen`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/isopen) |  Succeeds if the hive stash is currently open, fails otherwise.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Open`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/open) |  Open the hive stash. If a spawner is linked, trigger it and rescue the resulting `agent` from the hive stash.
  * When the `agent` is rescued, if _ShouldPlayRescuedAnimation_ is `true`, they will play a short animation to get off the hive stash.

  
[`RescueAgent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/rescueagent) |  Teleport `Agent` to the hive stash. If _ShouldPlayRescuedAnimation_ is `true`, they will play a short animation to get off the hive stash.
  * If the hive stash is closed, it will burst open first, ignoring a linked spawner. This will not clear a linked spawner.
  * Has no effect if `Agent` cannot be teleported for any reason, such as not being in the game anymore.

  
[`Reset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/reset) |  Reset the hive stash. This closes it, sets it to full health, and sets _ShouldPlayRescuedAnimation_ , _CanBeDamaged_ , _CanDropItems_ , _InteractTextOverride_ , and _HiveStashStyle_ to their initial values.
  * This can also clear the link to a spawner depending on _ShouldKeepLinkOnReset_.
  * The hive stash will not reset if the rescue animation is currently playing.

  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`SetHealth`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/sethealth) |  Set the device's current health.  
[`SetLinkedSpawner`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/setlinkedspawner) |  Link the hive stash to `GuardSpawner`. This overrides the hive stash's existing link if one exists.
  * If the hive stash opens with a linked spawner, trigger the spawner and rescue the resulting `agent` from the hive stash.
  * The rescued `agent` respects its spawner's events, functions, spawn count, and spawn limits.

  
[`SetLinkedSpawner`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/setlinkedspawner-1) |  Link the hive stash to `NPCSpawner`. This overrides the hive stash's existing link if one exists.
  * If the hive stash opens with a linked spawner, trigger the spawner and rescue the resulting `agent` from the hive stash.
  * The rescued `agent` respects its spawner's events, functions, spawn count, and spawn limits.

  
[`SetLinkedSpawnerFromAgent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/setlinkedspawnerfromagent) |  If `Agent` was spawned by a `guard_spawner_device` or `npc_spawner_device`, link the hive stash to that spawner. This overrides the hive stash's existing link if one exists.
  * Fails if `Agent` is not from a `guard_spawner_device` or `npc_spawner_device`, or if `Agent` was already eliminated.
  * If the hive stash opens with a linked spawner, trigger the spawner and rescue the resulting `agent` from the hive stash.
  * The rescued `agent` respects its spawner's events, functions, spawn count, and spawn limits.

  
[`SetMaxHealth`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device/setmaxhealth) |  Set the device's current maximum health.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device#inheritancehierarchy)
  * [Exposed Interfaces](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device#exposedinterfaces)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/hive_stash_device#functions)




