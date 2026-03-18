## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getaxis

# (Rotation:rotation).GetAxis extension
Learn technical details about the (Rotation:rotation).GetAxis extension.
Makes a `vector3` from the axis of `rotation` for an right-handed angle. If `rotation` is nearly identity, this will return the +Forward axis. See also `GetAngleRadians`.
|
---|---
Verse `using` statement | `using { /Verse.org/SpatialMath }`
`(Rotation:rotation).GetAxis<public><native>()<reads>:`[`vector3`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/vector3)
## Parameters
`GetAxis` takes the following parameters:
Name | Type | Description
---|---|---
`Rotation` | `rotation` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `GetAxis` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
