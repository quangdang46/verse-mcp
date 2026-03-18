## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/makerotationradians

# MakeRotationRadians function
Learn technical details about the MakeRotationRadians function.
Makes a `rotation` from `Axis` and `Angle` in radians using a right-handed sign convention (e.g. a positive rotation around Up takes Forward to Left).
|
---|---
Verse `using` statement | `using { /Verse.org/SpatialMath }`
`MakeRotationRadians<public><native>(Axis:vector3, Angle:float)<reads>:`[`rotation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/rotation)
## Parameters
`MakeRotationRadians` takes the following parameters:
Name | Type | Description
---|---|---
`Axis` | `vector3` |
`Angle` | `float` |
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `MakeRotationRadians` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.
### Effects
The following effects determine how `MakeRotationRadians` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
