## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/getvehicle



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
  4. (InCharacter:fort_character).GetVehicle extension


# (InCharacter:fort_character).GetVehicle extension
Learn technical details about the (InCharacter:fort_character).GetVehicle extension. 
On this page
Returns the `fort_vehicle` for `InCharacter`. Fails if `InCharacter` is not associated with a `fort_vehicle`.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Vehicles }`  
`(InCharacter:fort_character).GetVehicle<public><native>()<transacts><decides>:`[`fort_vehicle`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/fort_vehicle)
## Parameters
`GetVehicle` takes the following parameters:
Name | Type | Description  
---|---|---  
`InCharacter` | `fort_character` |   
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `GetVehicle` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ extension](https://dev.epicgames.com/community/search?query=extension)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/getvehicle#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/getvehicle#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/getvehicle#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/vehicles/getvehicle#effects)






---
