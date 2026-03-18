## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findoverlaphits-2

# (Entity:entity).FindOverlapHits extension
Learn technical details about the (Entity:entity).FindOverlapHits extension.
Find all objects in the scene that would overlap Volume if they were placed at GlobalTransform. NOTE: This entity defines the context(scene) for the query but does not otherwise take part in the sweep.
|
---|---
Verse `using` statement | `using { /Verse.org/SceneGraph }`
`(Entity:entity).FindOverlapHits<public>(GlobalTransform:transform, Volume:collision_volume)<transacts>:generator(overlap_hit)`
## Parameters
`FindOverlapHits` takes the following parameters:
Name | Type | Description
---|---|---
`Entity` | `entity` |
`GlobalTransform` | `transform` |
`Volume` | `collision_volume` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `FindOverlapHits` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.
