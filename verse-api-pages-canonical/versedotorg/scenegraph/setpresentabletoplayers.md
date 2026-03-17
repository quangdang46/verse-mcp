## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/setpresentabletoplayers



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
  4. (Entity:entity).SetPresentableToPlayers extension


# (Entity:entity).SetPresentableToPlayers extension
Learn technical details about the (Entity:entity).SetPresentableToPlayers extension. 
On this page
Assign the players that this entity will be presented to. False = presentable to everyone, array = presentable to no one.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SceneGraph }`  
`(Entity:entity).SetPresentableToPlayers<public><native>(Players:?[]player)<transacts><no_rollback>:void`
## Parameters
`SetPresentableToPlayers` takes the following parameters:
Name | Type | Description  
---|---|---  
`Entity` | `entity` |   
`Players` | `?[]player` |   
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `SetPresentableToPlayers` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
`no_rollback` | This is the default effect when no exclusive effect is specified. The `no_rollback` effect indicates that any actions performed by the function cannot be undone and so the function cannot be used in a failure context. This effect cannot be manually specified.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ extension](https://dev.epicgames.com/community/search?query=extension)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/setpresentabletoplayers#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/setpresentabletoplayers#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/setpresentabletoplayers#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/setpresentabletoplayers#effects)






---
