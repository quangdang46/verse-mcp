## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/tick_events



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
  4. tick_events class


# tick_events class
Learn technical details about the tick_events class. 
On this page
Describes discrete phases of a frame update. Subscribe to members of the tick_events object to run code before or after the physics system has updated your object, allowing you to affect or react to these updates.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SceneGraph }`  
## Members
This class has data members, but no functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`PrePhysics` | `execution_listenable` |  Listen `PrePhysics` to run your code before the physics system has updated your object this frame.  
`PostPhysics` | `execution_listenable` |  Listen `PostPhysics` to run your code after the physics system has updated your object this frame.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/tick_events#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/tick_events#data)






---
