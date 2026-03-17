## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creativeanimation/animation_controller_state



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
  4. animation_controller_state enumeration


# animation_controller_state enumeration
Learn technical details about the animation_controller_state enumeration. 
On this page
`animation_controller` states.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices/CreativeAnimation }`  
## Enumerators
The `animation_controller_state` enumeration includes the following enumerators:
Name | Description  
---|---  
`InvalidObject` |  The target of the animation is not an animatable prop. This could be because:
  * It is not a `creative_prop` that can be animated.
  * It was disposed or otherwise destroyed.
  * It has the 'Register with Structural Grid' option set in UEFN.

  
`AnimationNotSet` |  No animation has been successfully set via `animation_controller.SetAnimation`, or that animation has been cleared by a failed call to `animation_controller.SetAnimation`.  
`Stopped` |  Animation has either never started, finished, or was explicitly stopped.  
`Playing` |  Animation is playing.  
`Paused` |  Animation is paused.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ enumeration](https://dev.epicgames.com/community/search?query=enumeration)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Enumerators](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creativeanimation/animation_controller_state#enumerators)






---
