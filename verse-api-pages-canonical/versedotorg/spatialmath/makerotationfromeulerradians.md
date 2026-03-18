## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/makerotationfromeulerradians

# MakeRotationFromEulerRadians function
Learn technical details about the MakeRotationFromEulerRadians function.
Makes a `rotation` by applying a post-rotation of `LeftAxisAngle` followed by `UpAxisAngle` and then `ForwardAxisAngle `in that order. Right-handed convention (e.g. a positive rotation around Up takes +Forward to Left).
|
---|---
Verse `using` statement | `using { /Verse.org/SpatialMath }`
`MakeRotationFromEulerRadians<public><native>(LeftAxisAngle:float, UpAxisAngle:float, ForwardAxisAngle:float)<reads>:`[`rotation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/rotation)
## Parameters
`MakeRotationFromEulerRadians` takes the following parameters:
Name | Type | Description
---|---|---
`LeftAxisAngle` | `float` |
`UpAxisAngle` | `float` |
`ForwardAxisAngle` | `float` |
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `MakeRotationFromEulerRadians` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.
### Effects
The following effects determine how `MakeRotationFromEulerRadians` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
