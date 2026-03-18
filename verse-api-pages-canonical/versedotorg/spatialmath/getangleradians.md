## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/getangleradians

# (Rotation:rotation).GetAngleRadians extension
Learn technical details about the (Rotation:rotation).GetAngleRadians extension.
Returns the radians of right-handed `rotation` around the axis of `rotation`. See also `GetAxis`.
|
---|---
Verse `using` statement | `using { /Verse.org/SpatialMath }`
`(Rotation:rotation).GetAngleRadians<public><native>()<reads>:float`
## Parameters
`GetAngleRadians` takes the following parameters:
Name | Type | Description
---|---|---
`Rotation` | `rotation` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `GetAngleRadians` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
