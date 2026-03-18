## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/spatialmath/geteulerradians

# (Rotation:rotation).GetEulerRadians extension
Learn technical details about the (Rotation:rotation).GetEulerRadians extension.
Makes a `tuple(float, float, float)` with three elements:
  * _left axis_ `rotation` in radians
  * _up axis_ of `rotation` in radians
  * _forward axis_ of `rotation` in radians using the conventions of `MakeRotationEulerRadians`.

|
---|---
Verse `using` statement | `using { /Verse.org/SpatialMath }`
`(Rotation:rotation).GetEulerRadians<public><native>()<reads>:(float, float, float)`
## Parameters
`GetEulerRadians` takes the following parameters:
Name | Type | Description
---|---|---
`Rotation` | `rotation` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `GetEulerRadians` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
