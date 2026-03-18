## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/operatorstar-5

# operator'*' function
Learn technical details about the operator'*' function.
Makes a `vector3` by multiplying the components of `L` by `R`.
|
---|---
Verse `using` statement | `using { /UnrealEngine.com/Temporary/SpatialMath }`
`operator'*'<public>(L:vector3, R:float)<computes>:`[`vector3`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/spatialmath/vector3)
## Parameters
`operator'*'` takes the following parameters:
Name | Type | Description
---|---|---
`L` | `vector3` |
`R` | `float` |
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `operator'*'` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
### Effects
The following effects determine how `operator'*'` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning
---|---
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.
