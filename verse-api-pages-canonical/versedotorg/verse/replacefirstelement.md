## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/replacefirstelement

# (Input:[]t).ReplaceFirstElement extension
Learn technical details about the (Input:[]t).ReplaceFirstElement extension.
Makes an `array` by replacing the element at the lowest index that equals `ElementToReplace` with `ElementToReplaceWith` in `Input`. Fails if `Input` did not contain any instances of `ElementToReplace`.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`(Input:[]t).ReplaceFirstElement<public>(ElementToReplace:t, ElementToReplaceWith:t where t:comparable)<computes><decides>:[]t`
## Parameters
`ReplaceFirstElement` takes the following parameters:
Name | Type | Description
---|---|---
`Input` | `[]t` |
`ElementToReplace` | `t` |
`ElementToReplaceWith` | `t` |
`t` | `comparable` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `ReplaceFirstElement` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.
