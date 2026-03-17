## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arsinh



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
  4. ArSinh function


# ArSinh function
Learn technical details about the ArSinh function. 
On this page
Returns the inverse hyperbolic sine of `X` if `IsFinite(X)`.
|   
---|---  
Verse `using` statement | `using { /Verse.org/Verse }`  
`ArSinh<public><native>(X:float)<reads><computes>:float`
## Parameters
`ArSinh` takes the following parameters:
Name | Type | Description  
---|---|---  
`X` | `float` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `ArSinh` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
The following effects determine how `ArSinh` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`reads` | This effect indicates that the same inputs to the function may not always produce the same output. The behavior depends on factors external to the specified inputs, such as memory or the containing package version.  
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arsinh#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arsinh#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arsinh#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arsinh#effects)






---
