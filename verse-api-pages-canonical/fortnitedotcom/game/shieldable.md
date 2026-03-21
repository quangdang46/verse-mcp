## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable



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
  4. shieldable interface


# shieldable interface
Learn technical details about the shieldable interface. 
On this page
Implemented by Fortnite objects that have shields. A shield is a method of protection that can take incoming damage while leaving the health state unchanged.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Game }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`GetShield`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable/getshield) |  Returns the shield state of the object. This value will be between 0.0 and `MaxShield`  
[`SetShield`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable/setshield) |  Sets the shield state of the object.
  * Shield state will be clamped between 0.0 and `MaxShield`.

  
[`GetMaxShield`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable/getmaxshield) |  Returns the maximum shield state of the object. This value will be between 0.0 and Inf.  
[`SetMaxShield`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable/setmaxshield) |  Sets the maximum shield state of the object.
  * MaxShield will be clamped between 0.0 and Inf.
  * Current shield state will be scaled up or down based on the scale difference between the old and new MaxShield state.

  
[`DamagedShieldEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable/damagedshieldevent) |  Signaled when the shield is damaged.  
[`HealedShieldEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable/healedshieldevent) |  Signaled when the shield is healed.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable#functions)




