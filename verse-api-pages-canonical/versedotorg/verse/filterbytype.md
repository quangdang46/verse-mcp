## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/filterbytype



Table of Contents
# (InSet:classifiable_subset(element_type)).FilterByType extension
Learn technical details about the (InSet:classifiable_subset(element_type)).FilterByType extension. 
On this page
Returns a new set that contains all the elements in `InSet` that are of type `element_type`.
|   
---|---  
Verse `using` statement | `using { /Verse.org/Verse }`  
`(InSet:classifiable_subset(element_type)).FilterByType<public><native>(element_type:castable_subtype(k) where t:castable_subtype(k), k:any)<transacts>:`[`classifiable_subset(element_type)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/classifiable_subset/classifiable_subset\(element_type\))
## Parameters
`FilterByType` takes the following parameters:
Name | Type | Description  
---|---|---  
`InSet` | `classifiable_subset(element_type)` |   
`element_type` | `castable_subtype(k)` |   
`t` | `castable_subtype(k)` |   
`k` | `any` |   
## Attributes, Specifiers, and Effects
The following attributes, specifiers, and effects determine how you can interact with `FilterByType` in your programs, as well as how it behaves in your programs and UEFN. For the complete list of attributes, specifiers, and effects; see the [Specifiers Page](https://dev.epicgames.com/documentation/en-us/fortnite/specifiers-and-attributes-in-verse).
### Attributes
Attribute | Arguments | Meaning  
---|---|---  
`available` | `MinUploadedAtFNVersion := 3800` |   
`experimental` |  | This feature is in an experimental state, and you cannot publish projects implmenting it. The API for this feature is subject to change and backward compatibility is not guaranteed.  
### Specifiers
Specifier | Meaning  
---|---  
`public` | The identifier is universally accessible. You can use this on modules, classes, interfaces, structs, enums, methods, and data.  
`native` | Indicates that the definition details of the element are implemented in C++. Verse definitions with the `native` specifier auto-generate C++ definitions that a developer can then fill out its implementation. You can use this specifier on classes, interfaces, enums, methods, and data.  
### Effects
Effect | Meaning  
---|---  
`transacts` | This effect indicates that any actions performed by the function can be rolled back. The transacts effect is required any time a mutable variable (`var`) is written. You’ll be notified when you compile your code if the `transacts` effect was added to a function that can’t be rolled back. Note that this check is not done for functions with the `native` specifier.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ extension](https://dev.epicgames.com/community/search?query=extension)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Parameters](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/filterbytype#parameters)
  * [Attributes, Specifiers, and Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/filterbytype#attributes,specifiers,andeffects)
  * [Attributes](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/filterbytype#attributes)
  * [Specifiers](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/filterbytype#specifiers)
  * [Effects](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/filterbytype#effects)






---
