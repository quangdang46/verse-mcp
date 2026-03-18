## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/getlocalright

# (Rotation:rotation).GetLocalRight extension
Learn technical details about the (Rotation:rotation).GetLocalRight extension.
Makes a unit `vector3` pointing in the the local space _right_ direction in world space coordinates. This is equivalent to: `RotateVector(Rotation, vector3{X:=0.0, Y:=1.0, Z:=0.0})`.
|
---|---
Verse `using` statement | `using { /UnrealEngine.com/Temporary/SpatialMath }`
`(Rotation:rotation).GetLocalRight<public>()<transacts>:`[`vector3`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/vector3)
## Parameters
`GetLocalRight` takes the following parameters:
Name | Type | Description
---|---|---
`Rotation` | `rotation` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `GetLocalRight` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.
