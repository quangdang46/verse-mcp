## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device/gettransform



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
  4. GetTransform function


# GetTransform function
Learn technical details about the GetTransform function. 
On this page
Returns the transform of the `creative_device` with units in cm.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
`GetTransform<override>()<transacts>:`[`transform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/transform)
## Parameters
`GetTransform` does not take any parameters.
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `GetTransform` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`override` | Indicates that this child class provides a different method implementation than the parent class.  
### Effects
The following effects determine how `GetTransform` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device/gettransform#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device/gettransform#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device/gettransform#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device/gettransform#effects)






---
