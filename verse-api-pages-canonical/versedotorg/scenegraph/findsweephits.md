## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findsweephits



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
  4. (Entity:entity).FindSweepHits extension


# (Entity:entity).FindSweepHits extension
Learn technical details about the (Entity:entity).FindSweepHits extension. 
On this page
Find objects in the scene that would intersect this entity if it were swept from its location along the Displacement vector. Returns the first object interacting as collision_interaction.Block, and all objects interacting as collision_interaction.Overlap encountered before the first block. Hits are sorted by hit distance, so the blocking hit will be last.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SceneGraph }`  
`(Entity:entity).FindSweepHits<public>(Displacement:vector3)<transacts>:generator(sweep_hit)`
## Parameters
`FindSweepHits` takes the following parameters:
Name | Type | Description  
---|---|---  
`Entity` | `entity` |   
`Displacement` | `vector3` |   
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `FindSweepHits` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
### Effects
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ extension](https://dev.epicgames.com/community/search?query=extension)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findsweephits#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findsweephits#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findsweephits#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findsweephits#effects)






---
