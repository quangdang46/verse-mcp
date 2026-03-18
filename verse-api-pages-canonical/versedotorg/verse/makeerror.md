## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/makeerror

# MakeError function
Learn technical details about the MakeError function.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`MakeError<public>(Result:error_type where error_type:any)<transacts><no_rollback>:`[`result(success_type,error_type)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/result/result\(success_type,error_type\))
## Parameters
`MakeError` takes the following parameters:
Name | Type | Description
---|---|---
`Result` | `error_type` |
`error_type` | `any` |
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `MakeError` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
### Effects
The following effects determine how `MakeError` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning
---|---
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.
`no_rollback` | This is the default effect when no exclusive effect is specified. The `no_rollback` effect indicates that any actions performed by the function cannot be undone and so the function cannot be used in a failure context. This effect cannot be manually specified.
