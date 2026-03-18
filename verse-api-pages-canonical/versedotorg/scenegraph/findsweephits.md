## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/findsweephits

# (Entity:entity).FindSweepHits extension
Learn technical details about the (Entity:entity).FindSweepHits extension.
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
