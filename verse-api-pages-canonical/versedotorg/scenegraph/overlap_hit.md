## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/overlap_hit



Table of Contents
# overlap_hit struct
Learn technical details about the overlap_hit struct. 
On this page
The results of an overlap query. See entity.FindOverlapHits(). We will get one overlap_hit for each intersection of any volume in SourceVolumes with any other volume.
|   
---|---  
Verse `using` statement | `using { /Verse.org/SceneGraph }`  
## Members
This struct has data members, but no functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`SourceComponent` | `?component` |  The source component and volume (query input). For compound inputs (like an entity hierarchy) this will be a component/volume in that hierarchy. The SourceTransform is the transform of SourceVolume used for the overlap test. For single volume inputs like a sphere, the Source volume and transform are just the inputs to the overlap test, and the component is false.  
`SourceVolume` | `collision_volume` |  The source volume (query input)  
`SourceGlobalTransform` | `transform` |  The source volume transform  
`TargetComponent` | `component` |  The component that was hit by SourceVolume  
`TargetVolume` | `collision_element` |  The volume that was hit by SourceVolume  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ struct](https://dev.epicgames.com/community/search?query=struct)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/overlap_hit#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/overlap_hit#data)






---
