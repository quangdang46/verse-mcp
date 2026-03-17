## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/roamaround



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
  4. RoamAround function


# RoamAround function
Learn technical details about the RoamAround function. 
On this page
Roam around the current position. Use 'Tether' to specify the radius; otherwise, the Fortnite guard will roam anywhere.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/AI }`  
`RoamAround<public><native>(MovementType:movement_type)<transacts><suspends><no_rollback>:`[`result(success_type,error_type)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/result/result\(success_type,error_type\))
## Parameters
`RoamAround` takes the following parameters:
Name | Type | Description  
---|---|---  
`MovementType` | `movement_type` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `RoamAround` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
The following effects determine how `RoamAround` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
`suspends` | Indicates that the function is async. Creates an async context for the body of the function.  
`no_rollback` | This is the default effect when no exclusive effect is specified. The `no_rollback` effect indicates that any actions performed by the function cannot be undone and so the function cannot be used in a failure context. This effect cannot be manually specified.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/roamaround#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/roamaround#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/roamaround#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/roamaround#effects)






---
