## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection



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
  4. fort_team_collection interface


# fort_team_collection interface
Learn technical details about the fort_team_collection interface. 
On this page
Collection used to manage `team`s and `agent`s on those teams. Use `fort_playspace.GetTeamCollection()` to get the `team_collection` for the active experience.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Teams }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`GetTeams`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/getteams) |  Returns an array of all the `team`s known to this `fort_team_collection`  
[`AddToTeam`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/addtoteam) |  Adds `InAgent` to `InTeam`. Fails if `InTeam` is not part of the `fort_team_collection`.  
[`IsOnTeam`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/isonteam) |  Succeeds if `InAgent` is on `InTeam`. Fails if:
  * `InAgent` is not on `InTeam`.
  * `InTeam` is not part of the `fort_team_collection`.

  
[`GetAgents`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/getagents) |  Returns an array of all `agent`s on `InTeam`. Fails if `InTeam` is not part of the `fort_team_collection`.  
[`GetTeam`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/getteam) |  Get the `team` that `InAgent` is on. Fails if `InAgent` is not on a team in this `fort_team_collection`.  
[`GetTeamAttitude`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/getteamattitude) |  Returns the `team_attitude` between `Team1` and `Team2`. Fails if:
  * `Team1` is not in this `fort_team_collection`.
  * `Team2` is not in this `fort_team_collection`.

  
[`GetTeamAttitude`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection/getteamattitude-1) |  Returns the `team_attitude` between `Agent1` and `Agent2`. Fails if:
  * `Agent1` is not on a team in this `fort_team_collection`.
  * `Agent2` is not on a team in this `fort_team_collection`.

  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/teams/fort_team_collection#functions)




