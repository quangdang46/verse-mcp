## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/result

# result function
Learn technical details about the result function.
Implemented by classes that provide a result for an operation, which can fail or be successful
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`result<public>(success_type:any, error_type:any):result(success_type,error_type)`
This function is a parametric type, meaning it returns a class or interface rather than a value or object instance.
## Parameters
`result` takes the following parameters:
Name | Type | Description
---|---|---
`success_type` | `any` |
`error_type` | `any` |
### Generated Interface
`result` returns the parametric interface [`result(success_type,error_type)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/result/result\(success_type,error_type\)).
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `result` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
