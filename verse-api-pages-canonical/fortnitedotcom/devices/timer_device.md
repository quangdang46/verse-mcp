## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device



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
  4. timer_device class


# timer_device class
Learn technical details about the timer_device class. 
On this page
Provides a way to keep track of the time something has taken, either for scoreboard purposes, or to trigger actions. It can be configured in several ways, either acting as a countdown to an event that is triggered at the end, or as a stopwatch for an action that needs to be completed before a set time runs out.
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
`FailureEvent` | `listenable(payload)` |  Signaled when the timer completes or ends with failure. Sends the `agent` that activated the timer, if any.  
`StartUrgencyModeEvent` | `listenable(payload)` |  Signaled when the timer enters _Urgency Mode_. Sends the `agent` that activated the timer, if any.  
`SuccessEvent` | `listenable(payload)` |  Signaled when the timer completes or ends with success. Sends the `agent` that activated the timer, if any.  
### Functions
Function Name | Description  
---|---  
[`ClearPersistenceData`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/clearpersistencedata) |  Clears this device's saved data for `Agent`.  
[`ClearPersistenceDataForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/clearpersistencedataforall) |  Clears this device's saved data for all `agent`s.  
[`ClearPersistenceDataForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/clearpersistencedataforall-1) |  Clears this device's saved data for all `agent`s.  
[`Complete`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/complete) |  Completes the timer for `Agent`.  
[`Complete`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/complete-1) |  Completes the timer.  
[`CompleteForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/completeforall) |  Completes the timer for all `agent`s.  
[`CompleteForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/completeforall-1) |  Completes the timer for all `agent`s.  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/disable) |  Disables this device for `Agent`. While disabled this device will not receive signals.  
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/disable-1) |  Disables this device. While disabled this device will not receive signals.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/enable) |  Enables this device for `Agent`.  
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/enable-1) |  Enables this device.  
[`GetActiveDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/getactiveduration) |  Returns the remaining time (in seconds) on the timer for `Agent`.  
[`GetActiveDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/getactiveduration-1) |  Returns the remaining time (in seconds) on the timer if it is set to be global.  
[`GetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/getglobaltransform) |  Gets the global transform of this object.  
[`GetMaxDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/getmaxduration) |  Returns the maximum duration of the timer (in seconds).  
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.  
[`IsStatePerAgent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/isstateperagent) |  Succeeds if this device is tracking timer state for each individual `agent` independently. Fails if state is being tracked globally for all `agent`'s.  
[`Load`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/load) |  Loads this device's saved data for `Agent`.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.  
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-2) |  Moves the `creative_device` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_device` it will be stopped and put into the `AnimationNotSet` state.  
[`Pause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/pause) |  Pauses the timer for `Agent`.  
[`Pause`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/pause-1) |  Pauses the timer.  
[`PauseForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/pauseforall) |  Pauses the timer for all `agent`s.  
[`PauseForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/pauseforall-1) |  Pauses the timer for all `agent`s.  
[`Reset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/reset) |  Resets the timer back to its base time and stops it for `Agent`.  
[`Reset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/reset-1) |  Resets the timer back to its base time and stops it.  
[`ResetForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/resetforall) |  Resets the timer back to its base time and stops it for all `agent`s.  
[`ResetForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/resetforall-1) |  Resets the timer back to its base time and stops it for all `agent`s.  
[`Resume`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/resume) |  Resumes the timer for `Agent`.  
[`Resume`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/resume-1) |  Resumes the timer.  
[`ResumeForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/resumeforall) |  Resumes the timer for all `agent`s.  
[`ResumeForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/resumeforall-1) |  Resumes the timer for all `agent`s.  
[`Save`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/save) |  Saves this device's data for `Agent`.  
[`SetActiveDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/setactiveduration) |  Sets the remaining time (in seconds) on the timer, if active, on `Agent`.  
[`SetActiveDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/setactiveduration-1) |  Sets the remaining time (in seconds) on the timer, if active. Use this function if the timer is set to use the same time for all `agent`'s.  
[`SetGlobalTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/setglobaltransform) |  Sets the global transform of this object.  
[`SetLapTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/setlaptime) |  Sets the lap time indicator for `Agent`.  
[`SetLapTimeForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/setlaptimeforall) |  Sets the lap time indicator for all `agent`s.  
[`SetLapTimeForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/setlaptimeforall-1) |  Sets the lap time indicator for all `agent`s.  
[`SetMaxDuration`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/setmaxduration) |  Sets the maximum duration of the timer (in seconds).  
[`Start`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/start) |  Starts the timer for `Agent`.  
[`Start`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/start-1) |  Starts the timer.  
[`StartForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/startforall) |  Starts the timer for all `agent`s.  
[`StartForAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device/startforall-1) |  Starts the timer for all `agent`s.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-2) |  Teleports the `creative_device` to the specified location defined by `Transform`, also applies rotation and scale accordingly.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/timer_device#functions)




