## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/tick_events

# tick_events class
Learn technical details about the tick_events class.
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
