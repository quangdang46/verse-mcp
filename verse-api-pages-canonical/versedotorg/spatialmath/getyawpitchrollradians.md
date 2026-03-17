## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getyawpitchrollradians



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
  4. (Rotation:rotation).GetYawPitchRollRadians extension


# (Rotation:rotation).GetYawPitchRollRadians extension
Learn technical details about the (Rotation:rotation).GetYawPitchRollRadians extension. 
On this page
Makes a `tuple(float, float, float)` with three elements:
  * _yaw_ of `rotation` in radians
  * _pitch_ of `rotation` in radians
  * _roll_ of `rotation` in radians using the conventions of `MakeRotationFromYawPitchRollRadians`.


|   
---|---  
Verse `using` statement | `using { /Verse.org/SpatialMath }`  
`(Rotation:rotation).GetYawPitchRollRadians<public>()<reads><computes>:(float, float, float)`
## Parameters
`GetYawPitchRollRadians` takes the following parameters:
Name | Type | Description  
---|---|---  
`Rotation` | `rotation` |   
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `GetYawPitchRollRadians` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
### Effects
Effect | Meaning  
---|---  
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.  
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ extension](https://dev.epicgames.com/community/search?query=extension)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getyawpitchrollradians#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getyawpitchrollradians#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getyawpitchrollradians#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getyawpitchrollradians#effects)






---
