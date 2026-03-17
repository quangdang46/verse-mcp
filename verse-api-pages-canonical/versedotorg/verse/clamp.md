## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp



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
  4. Clamp function


# Clamp function
Learn technical details about the Clamp function. 
On this page
Constrains the value of `Val` between `A` and `B`. Robustly handles different argument orderings. Returns the median of `Val`, `A`, and `B`, such that comparisons with `NaN` operate as if `NaN > +Inf`.
|   
---|---  
Verse `using` statement | `using { /Verse.org/Verse }`  
`Clamp<public>(Val:float, A:float, B:float)<computes>:float`
## Parameters
`Clamp` takes the following parameters:
Name | Type | Description  
---|---|---  
`Val` | `float` |   
`A` | `float` |   
`B` | `float` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `Clamp` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
### Effects
The following effects determine how `Clamp` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`computes` | This effect requires that the function has no side effects, and is not guaranteed to complete. There’s an unchecked requirement that the function, when provided with the same arguments, produces the same result. Any function that doesn’t have the `native` specifier that would otherwise have the `converges` effect is a good example of using the `computes` effect.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp#effects)






---
