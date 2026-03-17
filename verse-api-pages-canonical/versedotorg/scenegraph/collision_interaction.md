## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/collision_interaction



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
  4. collision_interaction enumeration


# collision_interaction enumeration
Learn technical details about the collision_interaction enumeration. 
On this page
Specifies how a collision volume pair should interact. See collision_profile.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SceneGraph }`  
## Enumerators
The `collision_interaction` enumeration includes the following enumerators:
Name | Description  
---|---  
`Ignore` |  The pair will not be detected by Overlap and Sweep queries. The pair will not collide in the physics simulation.  
`Overlap` |  The pair will be detected by Overlap and Sweep queries. The pair will not collide in the physics simulation.  
`Block` |  The pair will be detected by Overlap and Sweep queries. The pair will collide in the physics simulation.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ enumeration](https://dev.epicgames.com/community/search?query=enumeration)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Enumerators](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/collision_interaction#enumerators)






---
