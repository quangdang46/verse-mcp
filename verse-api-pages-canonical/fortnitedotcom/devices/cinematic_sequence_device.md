## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device



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
  4. cinematic_sequence_device class


# cinematic_sequence_device class
Learn technical details about the cinematic_sequence_device class. 
On this page
Used to trigger level sequences that allow coordination of cinematic animation, transformation, and audio tracks.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description  
---|---  
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.  
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`StoppedEvent` | `listenable(payload)` |  Signaled when the sequence is stopped.  
### Functions
Function Name | Description  
---|---  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetPlaybackFrame`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/getplaybackframe) |  Returns the playback position (in frames) of the sequence.  
[`GetPlaybackTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/getplaybacktime) |  Returns the playback position (in time/seconds) of the sequence.  
[`GetPlayRate`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/getplayrate) |  Returns the playback rate of the sequence.  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`GoToEndAndStop`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/gotoendandstop) |  Go to the end and stop the sequence.  
[`GoToEndAndStop`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/gotoendandstop-1) |  Go to the end and stop the sequence. An instigating 'Agent' is required when the device is set to anything except _Everyone_.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Pause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/pause) |  Pauses the sequence.  
[`Pause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/pause-1) |  Pauses the sequence. An instigating 'Agent' is required when the device is set to anything except _Everyone_.  
[`Play`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/play) |  Plays the sequence. This will only work when the device is set to _Everyone_  
[`Play`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/play-1) |  Plays the sequence. An instigating 'Agent' is required when the device is set to anything except _Everyone_.  
[`PlayReverse`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/playreverse) |  Plays the sequence in reverse. This will only work when the device is set to _Everyone_  
[`PlayReverse`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/playreverse-1) |  Plays the sequence in reverse. An instigating 'Agent' is required when the device is set to anything except _Everyone_.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`SetPlaybackFrame`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/setplaybackframe) |  Set the playback position (in frames) of the sequence.  
[`SetPlaybackTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/setplaybacktime) |  Set the playback position (in time/seconds) of the sequence.  
[`SetPlayRate`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/setplayrate) |  Set the playback rate of the sequence.  
[`Stop`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/stop) |  Stops the sequence.  
[`Stop`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/stop-1) |  Stops the sequence. An instigating 'Agent' is required when the device is set to anything except _Everyone_.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TogglePause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/togglepause) |  Toggles between `Play` and `Stop`.  
[`TogglePause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device/togglepause-1) |  Toggles between `Play` and `Stop`. An instigating 'Agent' is required when the device is set to anything except _Everyone_.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/cinematic_sequence_device#functions)




