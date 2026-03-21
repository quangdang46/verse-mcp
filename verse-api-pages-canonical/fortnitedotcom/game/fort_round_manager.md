## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/fort_round_manager



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
  4. fort_round_manager interface


# fort_round_manager interface
Learn technical details about the fort_round_manager interface. 
On this page
This interface is implemented by the round manager living on the simulation entity.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Game }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`SubscribeRoundStarted`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/fort_round_manager/subscriberoundstarted) |  Subscribed callbacks will be invoked in two scenarios:
  * When a new Fortnite round starts
  * If a round is ongoing, your callback will be invoked immediately Upon the round ending, all callbacks will be canceled

  
[`SubscribeRoundEnded`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/fort_round_manager/subscriberoundended) |  Subscribed callbacks will be invoked when a Fortnite round ends  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/fort_round_manager#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/fort_round_manager#functions)




