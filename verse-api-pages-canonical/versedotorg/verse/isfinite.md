## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/isfinite



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
  4. (X:float).IsFinite extension


# (X:float).IsFinite extension
Learn technical details about the (X:float).IsFinite extension. 
On this page
Returns `X` if `X` is finite. Fails if `X` is one of:`
  * `+Inf`
  * `-Inf`
  * `NaN`


|   
---|---  
Verse `using` statement | `using { /Verse.org/Verse }`  
`(X:float).IsFinite<public>()<computes><decides>:float`
## Parameters
`IsFinite` takes the following parameters:
Name | Type | Description  
---|---|---  
`X` | `float` |   
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `IsFinite` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Specifiers
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
### Effects
Effect | Meaning  
---|---  
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.  
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ extension](https://dev.epicgames.com/community/search?query=extension)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/isfinite#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/isfinite#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/isfinite#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/isfinite#effects)






---
