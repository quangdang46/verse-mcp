## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creativeanimation/keyframe_delta



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
  4. keyframe_delta struct


# keyframe_delta struct
Learn technical details about the keyframe_delta struct. 
On this page
Instead of specifying the actual keyframe positions, we specify the keyframe _deltas_. This allows us to treat the initial position of the prop as keyframe 0 and avoid the question of how to get the prop to its initial location. For a `animation_mode.Loop` animation, the net rotation and translation must both be zero. Each delta is interpreted as a world-space transformation to be concatenated onto the previous transform(s).
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices/CreativeAnimation }`  
## Members
This struct has data members, but no functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`DeltaLocation` | `vector3` |  Target position of the `creative_prop`. This is a world-space coordinate in cm, with the initial position of the `creative_prop` acting as coordinate (0,0).  
`DeltaRotation` | `rotation` |  Target rotation for the `creative_prop`. Rotations are relative to the starting rotation of the `creative_prop`  
`DeltaScale` | `vector3` |  Target scale for the `creative_prop`. Scale is multiplicative to the starting Scale of the `creative_prop`  
`Time` | `float` |  Time in seconds the `creative_prop` should animate between its last frame and this frame.  
`Interpolation` | `cubic_bezier_parameters` |  Interpolation mode for this `keyframe_delta`. See `InterpolationTypes` for standard interpolation options. See `cubic_bezier_parameters` for authoring custom interpolations. Default = `InterpolationTypes.Linear`  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ struct](https://dev.epicgames.com/community/search?query=struct)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creativeanimation/keyframe_delta#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creativeanimation/keyframe_delta#data)




