## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator()-2

# operator'()' function
Learn technical details about the operator'()' function.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`operator'()'(Map:[t]v, Key:t where t:comparable, u:any, v:any)<reads><decides><predicts>:v`
## Parameters
`operator'()'` takes the following parameters:
Name | Type | Description
---|---|---
`Map` | `[t]v` |
`Key` | `t` |
`t` | `comparable` |
`u` | `any` |
`v` | `any` |
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `operator'()'` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Effects
The following effects determine how `operator'()'` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.
`predicts` |
