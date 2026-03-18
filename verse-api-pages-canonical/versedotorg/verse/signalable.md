## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/signalable

# signalable function
Learn technical details about the signalable function.
A parametric interface implemented by events with a `payload` that can be signaled. Can be used with `awaitable`, `subscribable`, or both (see: `listenable`).
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`signalable<public>(payload:any):signalable(payload)`
This function is a parametric type, meaning it returns a class or interface rather than a value or object instance.
## Parameters
`signalable` takes the following parameters:
Name | Type | Description
---|---|---
`payload` | `any` |
### Generated Interface
`signalable` returns the parametric interface [`signalable(payload)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/signalable/signalable\(payload\)).
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `signalable` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
