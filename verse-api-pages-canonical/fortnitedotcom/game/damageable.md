## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable



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
  4. damageable interface


# damageable interface
Learn technical details about the damageable interface. 
On this page
Implemented by Fortnite objects that can be damaged.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Game }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`Damage`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable/damage) |  Damage the `damageable` object anonymously by `Amount`. Setting `Amount` to less than 0 will cause no damage. Use `Damage(:damage_args):void` when damage is being applied from a known instigator and source.  
[`Damage`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable/damage-1) |  Damage the `damageable` object by `Args.Amount`. Setting `Amount` to less than 0 will cause no damage.  
[`DamagedEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable/damagedevent) |  Signaled when damage is applied to the `damageable` object.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable#functions)




