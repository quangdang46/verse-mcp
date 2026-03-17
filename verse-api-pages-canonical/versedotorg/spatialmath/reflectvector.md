## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/reflectvector



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
  4. ReflectVector function


# ReflectVector function
Learn technical details about the ReflectVector function. 
On this page
Makes a `vector3` by inverting the `SurfaceNormal` component of `Direction`.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SpatialMath }`  
`ReflectVector<public>(Direction:vector3, SurfaceNormal:vector3)<reads><computes>:`[`vector3`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/vector3)
## Parameters
`ReflectVector` takes the following parameters:
Name | Type | Description  
---|---|---  
`Direction` | `vector3` |   
`SurfaceNormal` | `vector3` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `ReflectVector` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
### Effects
The following effects determine how `ReflectVector` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.  
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/reflectvector#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/reflectvector#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/reflectvector#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/reflectvector#effects)






---
