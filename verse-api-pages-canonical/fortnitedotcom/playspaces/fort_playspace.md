## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace



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
  4. fort_playspace interface


# fort_playspace interface
Learn technical details about the fort_playspace interface. 
On this page
A nested container that scopes objects, style, gameplay rules, visuals, etc. All objects and players in an experience will belong to a fort_playspace. There is typically one `fort_playspace` for an entire experience, though this may change in the future as the platform evolves.
To access the `fort_playspace` for a `creative_device` use `creative_device.GetPlayspace`.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Playspaces }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`GetPlayers`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/getplayers) |  Get all human `player`s that are in the current `fort_playspace`.  
[`GetTeamCollection`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/getteamcollection) |  Get the `fort_team_collection` for the current `fort_playspace`.  
[`PlayerAddedEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/playeraddedevent) |  Signaled when a human `player` joins the `fort_playspace`. Returns a subscribable with a payload of the`player` that entered the `fort_playspace`.  
[`PlayerRemovedEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/playerremovedevent) |  Signaled when a human `player` leaves the `fort_playspace`. Returns a subscribable with a payload of the`player` that left the `fort_playspace`.  
[`GetParticipants`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/getparticipants) |  Get all `agent`s that are participating in the current `fort_playspace` experience. Participants might be human players (of `player` type) or AI-controlled characters (of `agent` type) that are registered and affecting the participant's count.  
[`ParticipantAddedEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/participantaddedevent) |  Signaled when a participant `agent` joins the `fort_playspace`. Returns a subscribable with a payload of the`agent` that entered the `fort_playspace`.  
[`ParticipantRemovedEvent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace/participantremovedevent) |  Signaled when a participant `agent` leaves the `fort_playspace`. Returns a subscribable with a payload of the`agent` that left the `fort_playspace`.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace#functions)




