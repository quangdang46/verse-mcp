## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damage_result



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
  4. damage_result struct


# damage_result struct
Learn technical details about the damage_result struct. 
On this page
Results for damage events on Fortnite objects.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Game }`  
## Members
This struct has data members, but no functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`Target` | `damageable` |  Object that was damaged.  
`Amount` | `float` |  Amount of damage applied to `Target`.  
`Instigator` | `?game_action_instigator` |  Player, agent, etc. that instigated the damage to `Target`. Can be false when damage is instigated by code, the environment, etc.  
`Source` | `?game_action_causer` |  Player, weapon, vehicle, etc. that damaged `Target`. Can be false when damage is caused by code, the environment, etc.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ struct](https://dev.epicgames.com/community/search?query=struct)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damage_result#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damage_result#data)




