## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/json/parse



Table of Contents
  1. ![Epic Games](https://edc-cdn.net/assets/images/logo-epic.svg)[Developer](https://dev.epicgames.com/)
  2. [Documentation](https://dev.epicgames.com/documentation/ "Documentation")
  3. Fortnite
     * [](https://dev.epicgames.com/documentation/en-us/unreal-engine)
     * [](https://dev.epicgames.com/documentation/en-us/fortnite)
     * [](https://dev.epicgames.com/documentation/en-us/twinmotion)
     * [](https://dev.epicgames.com/documentation/en-us/metahuman)
     * [](https://dev.epicgames.com/documentation/en-us/realityscan)
     * [](https://dev.epicgames.com/documentation/en-us/realityscan-mobile)
     * [](https://dev.epicgames.com/documentation/en-us/fab)
  4. Parse function


# Parse function
Learn technical details about the Parse function. 
On this page
Parse a JSON string returning a value with its contents
|   
---|---  
Verse `using` statement | `using { /UnrealEngine.com/JSON }`  
`Parse<public><native>(JSONString:[]char)<transacts><decides>:`[`value`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/json/value)
## Parameters
`Parse` takes the following parameters:
Name | Type | Description  
---|---|---  
`JSONString` | `[]char` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `Parse` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
The following effects determine how `Parse` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/json/parse#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/json/parse#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/json/parse#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/json/parse#effects)






---
