## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_ammo_component



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
  4. fort_inventory_ammo_component class


# fort_inventory_ammo_component class
Learn technical details about the fort_inventory_ammo_component class. 
On this page
`fort_inventory_ammo_component` is the inventory for Fortnite ammo.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Itemization }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `component`:
Name | Description  
---|---  
[`component`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component) |  Base class for authoring logic and data in the SceneGraph. Using components you can author re-usable building blocks of logic and data which can then be added to entities in the scene. Components are a very low level building block which can be used in many ways. For example:
  * Exposing engine level concepts like mesh or sound
  * Adding gameplay capabilities like damage or interaction
  * Storing an inventory for a character in the game

As components are generic there is no specific way that they must be used. It is up to the needs of your experience if you use one big game component or if you break up logic into many small components. Classes deriving from component must also specify `<final_super>` to be added to entities. This ensures the class will always derive directly from `component`. Further subclassing of the initial derived component is allowed and does not require specifying `<final_super>` on the derived classes. Only one instance of a component from each subclass group can be added to an entity at a time. For example, given this group of components, only one light_component can exist on a single entity. To create multiple lights you should use multiple entities. light_component := class(component){} capsule_light_component := class(light_component){} directional_light_component := class(light_component){} spot_light_component := class(light_component){} sphere_light_component := class(light_component){} rect_light_component := class(light_component){} ============================================================================== Component Lifetime Components move through a series of lifetime functions as they are added to entities, added to the scene, and begin running in the simulation. Components should override these methods to perform setup and run their simulation. As a component shuts down it will then move through shutdown version of these functions, giving users the opportunity to clean up any retained state on the component before it is disposed . Lifetime Methods: OnAddedToScene OnBeginSimulation -> OnSimulate OnEndSimulation OnRemovingFromScene ==============================================================================  
[`inventory_component`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component) |  Inventory components hold items. An entity with an inventory component can be considered to have an inventory. The inventory component controls which items can enter or exit. They also determine whether an item can be equipped.  
[`fort_inventory_component`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_component) |  `fort_inventory_component` is a shared class for all Fortnite inventories, with appropriate default UI and behavior.  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`AddItemEvent` | `listenable(payload)` |   
`Entity` | `entity` |  The parent entity of this component.
  * Components must have a parent entity pointer provided when being constructed.
  * Components cannot be moved between parents.

  
`EquipItemEvent` | `listenable(payload)` |   
`RemoveItemEvent` | `listenable(payload)` |   
`TickEvents` | `?tick_events` |  Set callbacks to `TickEvents.PrePhysics` and `TickEvents.PostPhysics` to receive per-frame updates before and after physics is updated on your object.  
`UnequipItemEvent` | `listenable(payload)` |   
### Functions
Function Name | Description  
---|---  
[`AddItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/additem) |  Adds the item to this inventory only, ignoring sub-inventories. Fails if no items could be added.  
[`AddItemDistribute`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/additemdistribute) |  Adds the item to this inventory, including sub-inventories. Fails if no items could be added.  
[`FindInventories`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/findinventories) |  Returns a list of all sub-inventories within this inventory and its descendant inventories.  
[`FindItems`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/finditems) |  Returns a list of all items within this inventory and its descendant inventories.  
[`FindItems`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/finditems-1) |  Returns a list of all items (of a specific type) within this inventory and its descendant inventories.  
[`GetEquippedItems`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/getequippeditems) |   
[`GetInventories`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/getinventories) |  Returns a list of sub-inventories of this inventory only.  
[`GetItemInSlot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_component/getiteminslot) |  Returns the item in the specified slot, if there is one.  
[`GetItems`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/getitems) |  Returns a list of items within this inventory only.  
[`GetItems`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/getitems-1) |  Returns a list of items (of a specific type) within this inventory only.  
[`GetMaxSlots`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_component/getmaxslots) |  Returns the total number of slots in this inventory.  
[`IsInScene`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/isinscene) |  Succeeds if the component is currently in the scene.
  * After `OnAddedToScene` is called this call succeeds.
  * After `OnRemovingFromScene` is called this call fails.

  
[`IsSimulating`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/issimulating) |  Succeeds if the component is currently simulating.
  * After `OnBeginSimulation` is called this call succeeds.
  * After `OnEndSimulation` is called this call fails.

  
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

  
[`RemoveFromEntity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/removefromentity) |  Removes the component from the entity.
  * Removed components are removed from the scene and can only be added back to the same entity.
  * Flows through `OnEndSimulation`-> `OnRemovingFromScene`.

  
[`RemoveItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/itemization/inventory_component/removeitem) |  Removes an item from this inventory or its descendent inventories. Fails if the item could not be removed.  
[`SendDown`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/scenegraph/component/senddown) |  Send a scene event to this component, invoking OnReceive. Return true to consume the event and halt propagation to the next entity.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_ammo_component#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_ammo_component#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_ammo_component#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/itemization/fort_inventory_ammo_component#functions)




