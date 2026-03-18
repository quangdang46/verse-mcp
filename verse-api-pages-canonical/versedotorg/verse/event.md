## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event

# event function
Learn technical details about the event function.
A _recurring_ , successively signaled parametric `event` with a `payload` allowing a simple mechanism to coordinate between concurrent tasks.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
`event<public>(t:any):event(t)`
This function is a parametric type, meaning it returns a class or interface rather than a value or object instance.
## Parameters
`event` takes the following parameters:
Name | Type | Description
---|---|---
`t` | `any` |
### Generated Class
`event` returns the parametric class [`event(t)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event/event\(t\)).
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `event` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning
---|---
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.
