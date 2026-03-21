## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable



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
  4. healable interface


# healable interface
Learn technical details about the healable interface. 
On this page
Implemented by Fortnite objects that can be healed.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Game }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`Heal`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable/heal) |  Heal the `healable` object anonymously by `Amount`. Setting `Amount` to less than 0 will cause no healing. Use `Heal(:healing_args):void` when healing is being applied from a known instigator and source.  
[`Heal`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable/heal-1) |  Cause `Args.Amount` damage to the `damageable` object. Setting `Amount` to less than 0 will cause no damage.  
[`HealedEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable/healedevent) |  Signaled when healing is applied to the `healable` object.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable#functions)




