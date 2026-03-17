## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/slerp



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
  4. Slerp function


# Slerp function
Learn technical details about the Slerp function. 
On this page
Used to perform spherical linear interpolation between `From` (when `Ratio = 0.0`) and `To` (when `Ratio = 1.0`). Expects `0.0 <= Ratio <= 1.0`.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SpatialMath }`  
`Slerp<public><native>(InitialRotation:rotation, FinalRotation:rotation, Ratio:float)<reads>:`[`rotation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/rotation)
## Parameters
`Slerp` takes the following parameters:
Name | Type | Description  
---|---|---  
`InitialRotation` | `rotation` |   
`FinalRotation` | `rotation` |   
`Ratio` | `float` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `Slerp` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
The following effects determine how `Slerp` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/slerp#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/slerp#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/slerp#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/slerp#effects)






---
