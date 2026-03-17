## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater



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
  4. operator'>' function


# operator'>' function
Learn technical details about the operator'>' function. 
On this page
|   
---|---  
Verse `using` statement | `using { /Verse.org/Verse }`  
`operator'>'(Lhs:int, Rhs:int)<decides>:int`
## Parameters
`operator'>'` takes the following parameters:
Name | Type | Description  
---|---|---  
`Lhs` | `int` |   
`Rhs` | `int` |   
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `operator'>'` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Effects
The following effects determine how `operator'>'` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`decides` | Indicates that the function can fail, and that calling this function is a [failable expression](https://dev.epicgames.com/documentation/en-us/fortnite/failure-in-verse#failableexpression). Function definitions with the `decides` effect must also have the `transacts` effect, which means the actions performed by this function can be rolled back (as if the actions were never performed), if there’s a failure anywhere in the function.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater#effects)






---
