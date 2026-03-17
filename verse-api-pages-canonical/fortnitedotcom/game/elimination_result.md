## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/elimination_result



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
  4. elimination_result struct


# elimination_result struct
Learn technical details about the elimination_result struct. 
On this page
Result data for `fort_character` elimination events.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Game }`  
## Members
This struct has data members, but no functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`EliminatedCharacter` | `fort_character` |  The `fort_character` eliminated from the match by `EliminatingCharacter`.  
`EliminatingCharacter` | `?fort_character` |  `fort_character` that eliminated `EliminatedCharacter` from the match. `EliminatingCharacter` will be false when `EliminatedCharacter` was eliminated through non-character actions, such as environmental damage.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ struct](https://dev.epicgames.com/community/search?query=struct)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/elimination_result#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/elimination_result#data)






---
