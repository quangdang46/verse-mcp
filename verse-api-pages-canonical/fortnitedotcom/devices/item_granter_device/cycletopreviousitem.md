## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletopreviousitem



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
  4. CycleToPreviousItem function


# CycleToPreviousItem function
Learn technical details about the CycleToPreviousItem function. 
On this page
Cycles to the previous item. If _Grant on Cycle_ is set `Agent` will be granted the item.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
`CycleToPreviousItem<public>(Agent:agent)<transacts><no_rollback>:void`
## Parameters
`CycleToPreviousItem` takes the following parameters:
Name | Type | Description  
---|---|---  
`Agent` | `agent` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `CycleToPreviousItem` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
### Effects
The following effects determine how `CycleToPreviousItem` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
`no_rollback` | This is the default effect when no exclusive effect is specified. The `no_rollback` effect indicates that any actions performed by the function cannot be undone and so the function cannot be used in a failure context. This effect cannot be manually specified.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletopreviousitem#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletopreviousitem#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletopreviousitem#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletopreviousitem#effects)






---
