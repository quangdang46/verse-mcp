## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/makeshortestrotationbetween



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
  4. MakeShortestRotationBetween function


# MakeShortestRotationBetween function
Learn technical details about the MakeShortestRotationBetween function. 
On this page
Makes the smallest angular `rotation` from `InitialRotation` to `FinalRotation` such that: `InitialRotation.RotateBy(MakeShortestRotationBetween(InitialRotation, FinalRotation)) = FinalRotation` and `MakeShortestRotationBetween(InitialRotation, FinalRotation)?.GetAngle()` is as small as possible.
|   
---|---  
Verse `using` statement | `using { /UnrealEngine.com/Temporary/SpatialMath }`  
`MakeShortestRotationBetween<public><native>(InitialRotation:rotation, FinalRotation:rotation)<transacts>:`[`rotation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/rotation)
## Parameters
`MakeShortestRotationBetween` takes the following parameters:
Name | Type | Description  
---|---|---  
`InitialRotation` | `rotation` |   
`FinalRotation` | `rotation` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `MakeShortestRotationBetween` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
The following effects determine how `MakeShortestRotationBetween` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/makeshortestrotationbetween#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/makeshortestrotationbetween#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/makeshortestrotationbetween#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/makeshortestrotationbetween#effects)






---
