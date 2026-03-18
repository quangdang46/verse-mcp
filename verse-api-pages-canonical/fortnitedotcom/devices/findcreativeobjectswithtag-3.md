## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/findcreativeobjectswithtag-3

# (InEntity:entity).FindCreativeObjectsWithTag extension
Learn technical details about the (InEntity:entity).FindCreativeObjectsWithTag extension.
Generates a `creative_object_interface` for every creative object that has been marked with the specified `Tag`. Will not find anything if called on an `entity` that is not in the scene.
|
---|---
Verse `using` statement | `using { /Fortnite.com/Devices }`
`(InEntity:entity).FindCreativeObjectsWithTag<public><native>(Tag:tag)<transacts>:generator(creative_object_interface)`
## Parameters
`FindCreativeObjectsWithTag` takes the following parameters:
Name | Type | Description
---|---|---
`InEntity` | `entity` |
`Tag` | `tag` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `FindCreativeObjectsWithTag` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.
