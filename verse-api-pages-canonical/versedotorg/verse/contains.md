## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/contains

# (InSet:classifiable_subset(element_type)).Contains extension
Learn technical details about the (InSet:classifiable_subset(element_type)).Contains extension.
Succeeds if `element_type` is present in `InSet`.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`(InSet:classifiable_subset(element_type)).Contains<public>(element_type:castable_subtype(k) where t:castable_subtype(k), k:any)<reads><computes><decides>:void`
## Parameters
`Contains` takes the following parameters:
Name | Type | Description
---|---|---
`InSet` | `classifiable_subset(element_type)` |
`element_type` | `castable_subtype(k)` |
`t` | `castable_subtype(k)` |
`k` | `any` |
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `Contains` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Attributes
Attribute | Arguments | Meaning
---|---|---
`available` | `MinUploadedAtFNVersion := 3800` |
`experimental` |  | This feature is in an experimental state, and you cannot publish projects implmenting it. The API for this feature is subject to change and backward compatibility is not guaranteed.
### Specifiers
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
### Effects
Effect | Meaning
---|---
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.
