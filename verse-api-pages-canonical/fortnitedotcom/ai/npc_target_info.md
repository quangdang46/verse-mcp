## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_target_info

# npc_target_info class
Learn technical details about the npc_target_info class.
Information about a perceived target.
|
---|---
Verse `using` statement | `using { /Fortnite.com/AI }`
## Members
This class has data members, but no functions.
### Data
Data Member Name | Type | Description
---|---|---
`Target` | `entity` |  The entity that was detected.
`HasLineOfSight` | `?logic` |  True if the target can be seen.
`Attitude` | `?team_attitude` |  Attitude toward this target.
`LastKnownPosition` | `?vector3` |  Last known position of this target.
`OnUpdateEvent` | `listenable(payload)` |
