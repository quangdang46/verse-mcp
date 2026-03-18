## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creativeanimation/animation_mode

# animation_mode enumeration
Learn technical details about the animation_mode enumeration.
Animation play modes.
|
---|---
Verse `using` statement | `using { /Fortnite.com/Devices/CreativeAnimation }`
## Enumerators
The `animation_mode` enumeration includes the following enumerators:
Name | Description
---|---
`OneShot` |  Stop after playing the animation once.
`PingPong` |  Reverse direction after reaching the final `keyframe_delta`, then play the animation in reverse.
`Loop` |  Play the animation in a loop. This requires the animation ends exactly where it began.
