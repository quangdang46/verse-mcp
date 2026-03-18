## https://dev.epicgames.com/documentation/en-us/fortnite/working-with-entities-and-components-in-unreal-editor-for-fortnite

# Working with Entities and Components
Learn how to build out entities and add data and behaviors to them with components.
![Working with Entities and Components](https://dev.epicgames.com/community/api/documentation/image/41706323-ef99-4701-a23c-68721d417e57?resizing_type=fill&width=1920&height=335)
Learn to use this **Beta** feature, but use caution when shipping with it.
Below are the workflows for adding entities, [components](https://dev.epicgames.com/documentation/en-us/fortnite/component), and [prefabs](https://dev.epicgames.com/documentation/en-us/fortnite/prefabs-and-prefab-instances-in-unreal-editor-for-fortnite) to your project. You can use these workflows to create complex objects and add them to your experience.
##  Creating an Entity
To create an entity:
  1. Open the **Place Actors** panel.
  2. Select the **Entity** icon.
[![](https://dev.epicgames.com/community/api/documentation/image/9d9f351f-4380-4581-b5ad-92ff6072cf52?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/9d9f351f-4380-4581-b5ad-92ff6072cf52?resizing_type=fit)
  3. Drag an empty entity into the [viewport](https://dev.epicgames.com/documentation/en-us/uefn/user-interface-reference-for-unreal-editor-for-fortnite#Viewport).
You can begin adding components to the entity from the [Details panel](https://dev.epicgames.com/documentation/en-us/fortnite/unreal-editor-for-fortnite-glossary).

##  Adding More Entities Through the Outliner
Once you have an entity in the scene, you can add more entities through the Outliner.
Duplicating the entity creates more entities in the Outliner.
![](https://dev.epicgames.com/community/api/documentation/image/5150f106-0e22-4c97-8606-8efdd07bd0aa?resizing_type=fit)
You can also right-click on an entity in the Outliner and choose the following options:
  * **Add Entity** : This option adds a new entity nested under the entity you originally selected.
  * **Group Under New Entity** : This option creates a new entity that becomes the parent of the entity you originally selected. If your original entity was nested under another entity, the structure stays the same, with the new entity inserted between the original parent and child entities.

##  Nesting and Structuring Entities
Placing entities into a hierarchical structure creates relationships between the entities in the hierarchy. The nesting structure has four levels:
  * **Ancestor** : Any level above parent (parent, grandparent, great-grandparent, and so on).
  * **Descendent** : Any level below the currently selected entity (children, grandchildren, and so on).
  * **Parent** : Single ancestor one level above the currently selected entity.
  * **Child** : Descendent one level below the currently selected entity.

In this structure, the Ancestor entity controls the lifetime of all the entities nested underneath.
[![An example of nested entities.](https://dev.epicgames.com/community/api/documentation/image/f91b9004-05a8-47e3-8655-5812c2f3ad09?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f91b9004-05a8-47e3-8655-5812c2f3ad09?resizing_type=fit)
In the example below, the **lamp post** on its own is a simple game object that can only do the one thing its component is set to do: be a static mesh of a lamp post.
[![](https://dev.epicgames.com/community/api/documentation/image/96ec7bda-14f1-447e-95db-0f1f0d2c93b5?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/96ec7bda-14f1-447e-95db-0f1f0d2c93b5?resizing_type=fit)
When the **Rotation_Point** entity, **Dock_Lantern** entity, and **SpotLight** entity are nested beneath the **Default_Wooden_LightPost_Prefab_C** entity, the lamp post takes on the characteristics of its children, making the lamp post a more complex game object.
[![](https://dev.epicgames.com/community/api/documentation/image/cf1b5842-716d-481e-8d76-b3644d0e1edd?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/cf1b5842-716d-481e-8d76-b3644d0e1edd?resizing_type=fit)
There are a few reasons to do this:
  * Scale the component functions of an entity to specify how the entity should work in the scene.
  * Determine how child and descendent entities interact with the parent and ancestor entities.
  * Hierarchical structuring represents object lifetime in-game; if the ancestor object is destroyed, then so are all the descendent and child objects.
When grouping entities, rename the parent entity so you know what the entity is.

Nesting is a concept that you can take advantage of. For example, you can align architectural assets to reduce the likeliness of gaps in your buildings.
Descendent entities can be offset relative to the ancestor’s position in the world allowing you to set a central pivot point that controls the placement of the building’s walls, floors, and more when being duplicated or moved in the scene.
##  Adding a Component
Adding a component to an entity determines the behavior of game objects in your project.
To add components to your entity and customize them from the **Details** panel:
  1. Select the entity in the viewport or Outliner.
  2. In the **Details** panel of the entity, select **Add Component**.
[![](https://dev.epicgames.com/community/api/documentation/image/d51f24cb-fd63-49f8-af08-bc7a5505e514?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/d51f24cb-fd63-49f8-af08-bc7a5505e514?resizing_type=fit)
  3. Search through the **Component dropdown menu** , and select a component. This adds a component to the entity, an entry field for that component appears in the Details panel for the entity.
  4. From the **Details** panel, customize the component in the **component options**.
[![The directional_light_component user options in the Details panel.](https://dev.epicgames.com/community/api/documentation/image/4eb3a147-25c6-4b29-aaa9-4566d7c84f8f?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4eb3a147-25c6-4b29-aaa9-4566d7c84f8f?resizing_type=fit)

Components can be simple, like a mesh, or complex, like a custom Verse script. You can assign multiple components to one entity to define how that entity behaves in your project. However, only one component type can be used on an entity at a time.
This means once you select a component type you cannot reuse that component type on the same entity. To use the same component you'll need to add another entity and add the same component to the new entity. For more information on which components are available in Scene Graph, refer to [Components](https://dev.epicgames.com/documentation/en-us/fortnite/components-in-unreal-editor-for-fortnite).
Don’t see the component you need? Try making your own! Check out how to [create your own components with Verse](https://dev.epicgames.com/documentation/en-us/fortnite/creating-your-own-component-using-verse-in-unreal-editor-for-fortnite).
Currently you can only add one of a given component subclass. For example, you can only have one `point_light_component` on your entity, but you can have one `point_light_component` and one `rect_light_component` on your entity. The same limitation applies to your custom components made in Verse.
![](https://dev.epicgames.com/community/api/documentation/image/ec2e9d7d-7afa-40d9-aab1-80dbfa1d3597?resizing_type=fit)
###  Asset-Generated Components
An asset-generated component is a component class that is automatically created based on preexisting content in your project, such as a mesh, sound, or particle system asset. These assets may also expose properties that you can modify on the generated component.
##  Overriding Components
You can override components in the **Details** panel of the [Prefab Editor](https://dev.epicgames.com/documentation/en-us/fortnite/prefab-editor-user-interface-in-unreal-editor-for-fortnite) or in the scene. This means that you can change the nature of the component by adding new functionality or changing the associated assets of the component without having to create a new entity or remove the current component.
Components can have 4 different override states. These states are visible on the component card:
Image  |  Name  |  Description
---|---|---
[![](https://dev.epicgames.com/community/api/documentation/image/8dbdcc18-bcc8-4f9a-93d3-0e0aed6b5c2b?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/8dbdcc18-bcc8-4f9a-93d3-0e0aed6b5c2b?resizing_type=fit) |  **No Override** |  The component does not have an override.
[![](https://dev.epicgames.com/community/api/documentation/image/c1af527a-7ec3-4b16-9594-b4251bf9f1d8?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/c1af527a-7ec3-4b16-9594-b4251bf9f1d8?resizing_type=fit) |  **Override Here** |  The component is overridden here at this level.
[![](https://dev.epicgames.com/community/api/documentation/image/50e5be78-1ac8-42d7-8647-a8ead0b5dfb6?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/50e5be78-1ac8-42d7-8647-a8ead0b5dfb6?resizing_type=fit) |  **Override Inside** |  The component has an override state on one of its options. [INCLUDE:#state]
[![](https://dev.epicgames.com/community/api/documentation/image/720ea1a9-fe7d-4c8e-9c63-a70240da1396?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/720ea1a9-fe7d-4c8e-9c63-a70240da1396?resizing_type=fit) |  **Unique Override** |  The override is unique to this prefab instance. [INCLUDE:#state]
To override a component, do the following:
  1. In the **Details** panel for the component, click the **component card dropdown menu button** , which is a plus (**+**).
[![](https://dev.epicgames.com/community/api/documentation/image/31639bee-68a5-4506-b7bf-3c43ccf30b33?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/31639bee-68a5-4506-b7bf-3c43ccf30b33?resizing_type=fit)
  2. In the dropdown menu, select **Override Component**.
[![](https://dev.epicgames.com/community/api/documentation/image/9eed2425-cea2-4a95-9abe-5da868b0c828?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/9eed2425-cea2-4a95-9abe-5da868b0c828?resizing_type=fit)

In the image below, all empty icons next to the components in the Details panel now have the override icon.
[![](https://dev.epicgames.com/community/api/documentation/image/734ab82f-75df-4171-92b2-c1ff97e53c63?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/734ab82f-75df-4171-92b2-c1ff97e53c63?resizing_type=fit)
Changing the default values of a component’s options also creates an override to that component’s function, such as increasing the default values on a light component. When you change default values, the component control button features a **+ icon** to signal that default values were changed.
[![](https://dev.epicgames.com/community/api/documentation/image/6c59e864-4b44-405c-b19a-1ef9e0c305c0?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/6c59e864-4b44-405c-b19a-1ef9e0c305c0?resizing_type=fit)
##  Clear Overrides
To go back to the original prefab design, do the following:
  1. In the **Details** panel, click on the **component card dropdown menu button**.
[![](https://dev.epicgames.com/community/api/documentation/image/61d1e558-b3ad-4d1a-8f83-5905ba2c192b?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/61d1e558-b3ad-4d1a-8f83-5905ba2c192b?resizing_type=fit)
  2. Select **Clear Override**. The prefab returns to its original state.
[![](https://dev.epicgames.com/community/api/documentation/image/a1d13d3d-b3b6-427d-82fe-123c5c1708c8?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/a1d13d3d-b3b6-427d-82fe-123c5c1708c8?resizing_type=fit)

##  Removing a Component
To remove components from entities:
  1. In the **Details** panel, click the **component card dropdown menu button** , which is a plus (**+**), on the component.
[![](https://dev.epicgames.com/community/api/documentation/image/a04ae634-05bc-4ea6-98ac-c87c0d4a9388?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/a04ae634-05bc-4ea6-98ac-c87c0d4a9388?resizing_type=fit)
  2. In the dropdown menu, select **Remove Component**. The component is removed from the entity.
[![](https://dev.epicgames.com/community/api/documentation/image/79704b04-f509-463a-98a0-de4cdfa55641?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/79704b04-f509-463a-98a0-de4cdfa55641?resizing_type=fit)

[![](https://dev.epicgames.com/community/api/documentation/image/db18760d-7b43-47b0-828e-122f0b40c232?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/db18760d-7b43-47b0-828e-122f0b40c232?resizing_type=fit)
This workflow works in the Prefab Editor as well.
##  Saving Entities as Prefabs
Once you create your entities and add components to them, you can save specific entities as a prefab. This means you can create multiple instances of the same entity and component structure, and that these changes propagate across them all instantly. To learn how to create prefabs and propagate changes, see [Prefabs and Prefab Instances](https://dev.epicgames.com/documentation/en-us/fortnite/prefabs-and-prefab-instances-in-unreal-editor-for-fortnite).
##  Working with Scene Graph in Fortnite Creative
**Scene Graph Entity** support is available in Fortnite Creative. Using the **[Phone Tool](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary#phone)** you have basic editing capabilities in Scene Graph using the controls and feedback you're familiar with during **[Edit Mode](https://dev.epicgames.com/documentation/en-us/fortnite/unreal-editor-for-fortnite-glossary#edit-mode)**. The support feature also means you can interact with all entities and actors simultaneously in the scene using the Phone Tool.
These basic interactions between Fortnite Creative and Scene Graph are possible because entire entity hierarchies are selected as one object by the Phone Tool.
Scene Graph entities cannot be created in Creative edit Mode.
