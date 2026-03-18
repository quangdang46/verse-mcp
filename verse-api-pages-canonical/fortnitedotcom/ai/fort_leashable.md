## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/fort_leashable

# fort_leashable interface
Learn technical details about the fort_leashable interface.
|
---|---
Verse `using` statement | `using { /Fortnite.com/AI }`
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description
---|---
[`SetLeashPosition`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/fort_leashable/setleashposition) |  Set custom leash position. 'InnerRadius' ranges from 0.0 to 20000.0 (in centimeters). 'OuterRadius' ranges from 0.0 to 20000.0 (in centimeters) and no less than 'InnerRadius'.
[`SetLeashAgent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/fort_leashable/setleashagent) |  Set the agent to be the new center of the leash. 'InnerRadius' ranges from 0.0 to 20000.0 (in centimeters). 'OuterRadius' ranges from 0.0 to 20000.0 (in centimeters) and no less than 'InnerRadius'.
[`ClearLeash`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/fort_leashable/clearleash) |  Removes the current leash.
