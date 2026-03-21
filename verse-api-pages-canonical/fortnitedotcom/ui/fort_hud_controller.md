## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller



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
  4. fort_hud_controller interface


# fort_hud_controller interface
Learn technical details about the fort_hud_controller interface. 
On this page
A HUD controller that allows for showing and hiding of HUD elements.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/UI }`  
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description  
---|---  
[`ShowElements`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller/showelements) |  Shows a set of HUD elements for every player. Note: This can be overridden by rules set by 'ForPlayer' functions since player specific rules are prioritized over general rules.  
[`HideElements`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller/hideelements) |  Hides a set of HUD elements for every player. Note: This can be overridden by rules set by 'ForPlayer' functions since player specific rules are prioritized over general rules.  
[`ResetElementVisibility`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller/resetelementvisibility) |  Resets the visibility for a set of HUD elements for every player. Note: This will not clear player specific rules set by the 'ForPlayer' functions which can only be reset by calling 'ResetElementsForPlayer'.  
[`ShowElementsForPlayer`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller/showelementsforplayer) |  Shows a set of HUD elements for a single player. Note: This overrides general rules set by non-player functions for the given elements. Call 'ResetElementsForPlayer' in order to return the player to general behavior  
[`HideElementsForPlayer`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller/hideelementsforplayer) |  Hides a set of HUD elements for a single player. Note: This overrides general rules set by non-player functions for the given elements. Call 'ResetElementsForPlayer' in order to return the player to general behavior  
[`ResetElementsForPlayer`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller/resetelementsforplayer) |  Resets the player-specific visibility rules of a set of HUD elements for a single player. Note: This will not reset rules that have been set by means other than the 'PerPlayer' functions.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller#members)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/fort_hud_controller#functions)




