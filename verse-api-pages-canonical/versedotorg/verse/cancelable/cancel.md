## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable/cancel



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
  4. Cancel function


# Cancel function
Learn technical details about the Cancel function. 
On this page
Prevents any current or future work from completing.
|   
---|---  
Verse `using` statement | `using { /Verse.org/Verse }`  
`Cancel<public><native_callable>()<transacts>:void`
## Parameters
`Cancel` does not take any parameters.
## Attributes, Specifiers, and Effects
### Specifiers
The following specifiers determine how you can interact with `Cancel` in your programs. For the complete list of specifiers, see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native_callable` | Indicates that an instance method is both native (implemented in C++) and may be called by other C++ code. You can see this specifier used on an instance method. This specifier doesn’t propagate to subclasses and so you don’t need to add it to a definition when overriding a method that has this specifier.  
### Effects
The following effects determine how `Cancel` behaves in your programs. For the complete list of effects, see the Effect Specifers section of the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ function](https://dev.epicgames.com/community/search?query=function)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable/cancel#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable/cancel#attributes,specifiers,andeffects)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable/cancel#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable/cancel#effects)






---
