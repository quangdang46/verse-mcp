## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component



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
  4. guard_actions_component class


# guard_actions_component class
Learn technical details about the guard_actions_component class. 
On this page
Fortnite Guards AI actions management
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/AI }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `component`:
Name | Description  
---|---  
[`component`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component) |  Base class for authoring logic and data in the SceneGraph. Using components you can author re-usable building blocks of logic and data which can then be added to entities in the scene. Components are a very low level building block which can be used in many ways. For example:
  * Exposing engine level concepts like mesh or sound
  * Adding gameplay capabilities like damage or interaction
  * Storing an inventory for a character in the game

As components are generic there is no specific way that they must be used. It is up to the needs of your experience if you use one big game component or if you break up logic into many small components. Classes deriving from component must also specify `<final_super>` to be added to entities. This ensures the class will always derive directly from `component`. Further subclassing of the initial derived component is allowed and does not require specifying `<final_super>` on the derived classes. Only one instance of a component from each subclass group can be added to an entity at a time. For example, given this group of components, only one light_component can exist on a single entity. To create multiple lights you should use multiple entities. light_component := class(component){} capsule_light_component := class(light_component){} directional_light_component := class(light_component){} spot_light_component := class(light_component){} sphere_light_component := class(light_component){} rect_light_component := class(light_component){} ============================================================================== Component Lifetime Components move through a series of lifetime functions as they are added to entities, added to the scene, and begin running in the simulation. Components should override these methods to perform setup and run their simulation. As a component shuts down it will then move through shutdown version of these functions, giving users the opportunity to clean up any retained state on the component before it is disposed . Lifetime Methods: OnAddedToScene OnBeginSimulation -> OnSimulate OnEndSimulation OnRemovingFromScene ==============================================================================  
[`npc_actions_component`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component) |  Fortnite NPC AI actions management  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`Entity` | `entity` |  The parent entity of this component.
  * Components must have a parent entity pointer provided when being constructed.
  * Components cannot be moved between parents.

  
`MovementSpeedMultiplier` | `?float` |  Multiplier on the movement speed (value is clamped between 0.5 and 2).  
`TickEvents` | `?tick_events` |  Set callbacks to `TickEvents.PrePhysics` and `TickEvents.PostPhysics` to receive per-frame updates before and after physics is updated on your object.  
### Functions
Function Name | Description  
---|---  
[`Attack`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/attack) |  Attack the target. The target must have been detected.  
[`Crouch`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/crouch) |  Change stance to crouch. Will never complete unless interrupted.  
[`Focus`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component/focus) |  Look at the specified location. If LockFocus is false, the action stops once the NPC is facing the position.  
[`Focus`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component/focus-1) |  Look at the specified Entity. If LockFocus is false, the action stops once the NPC is facing the entity.  
[`GetCurrentDestination`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component/getcurrentdestination) |  Return the current destination of the character.  
[`Idle`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component/idle) |  Stay idle for a specific duration.  
[`IsInScene`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/isinscene) |  Succeeds if the component is currently in the scene.
  * After `OnAddedToScene` is called this call succeeds.
  * After `OnRemovingFromScene` is called this call fails.

  
[`IsSimulating`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/issimulating) |  Succeeds if the component is currently simulating.
  * After `OnBeginSimulation` is called this call succeeds.
  * After `OnEndSimulation` is called this call fails.

  
[`Jump`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/jump) |  Trigger a jump.  
[`MoveInRangeToAttack`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/moveinrangetoattack) |  Move in range to attack the current target.  
[`NavigateTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component/navigateto) |  Navigate toward the specified navigation target.  
[`OnAddedToScene`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/onaddedtoscene) |  Called when the component is added to the scene by parenting it under the simulation entity or another entity already in the scene.
  * Querying for components in the scene is valid after this phase completes.

  
[`OnBeginSimulation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/onbeginsimulation) |  Called when the component begins simulating within the scene.
  * Use this to set up TickEvent callbacks or other setup that must be guaranteed to complete immediately.
  * `OnAddedToScene` is guaranteed to run before `OnBeginSimulation`.

  
[`OnEndSimulation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/onendsimulation) |  Called when the component ends simulation within the scene.
  * Simulation ends on a component when the experience resets, the parent entity is removed from the scene.
  * Cached TickEvents cancelables should be canceled in `OnEndSimulation`.
  * `OnSimulate` task will be canceled before `OnEndSimulation` is called.
  * `OnEndSimulation` is only called on components that have already had `OnBeginSimulation` called.

  
[`OnReceive`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/onreceive) |  Respond to a scene event. Return true to consume the event and halt propagation to the next entity.  
[`OnRemovingFromScene`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/onremovingfromscene) |  Called when the component is about to be removed from the scene.
  * Components are removed from a scene when the parent entity is removed from the scene.
  * `OnRemovingFromScene` is only called on components that have already had `OnAddedToScene` called.

  
[`OnSimulate`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/onsimulate) |  Called when the component begins simulating within the scene.
  * Use this to add asynchronous/suspends update logic for a component.
  * `OnBeginSimulation` is guaranteed to run before `OnSimulate`.
  * `OnSimulate` will be cancelled before `OnEndSimulation`

  
[`PlayRandomEmote`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/playrandomemote) |  Play a random emote from the character emotes bank.  
[`RemoveFromEntity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/removefromentity) |  Removes the component from the entity.
  * Removed components are removed from the scene and can only be added back to the same entity.
  * Flows through `OnEndSimulation`-> `OnRemovingFromScene`.

  
[`Revive`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/revive) |  Revive the specified target.  
[`RoamAround`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/roamaround) |  Roam around the current position. Use 'Tether' to specify the radius; otherwise, the Fortnite guard will roam anywhere.  
[`SendDown`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/senddown) |  Send a scene event to this component, invoking OnReceive. Return true to consume the event and halt propagation to the next entity.  
[`Slide`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/slide) |  Trigger a slide.  
[`StandUp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/standup) |  Go back to standing.  
[`StopNavigation`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/npc_actions_component/stopnavigation) |  Stop navigation.  
[`Tether`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/tether) |  Tether the NPC to a position. 'Radius' is in centimeters.  
[`Tether`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/tether-1) |  Tether the NPC to an entity. 'Radius' is in centimeters.  
[`Untether`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component/untether) |  Untether the NPC.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ai/guard_actions_component#functions)




