## https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite



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
  4. Transforms in Scene Graph


# Transforms in Scene Graph
Transform Scene Graph entities to manipulate their translation, rotation, and scale. 
![Transforms in Scene Graph](https://dev.epicgames.com/community/api/documentation/image/84c814b5-9a73-4570-a2b4-c0e76387269f?resizing_type=fill&width=1920&height=335)
On this page
The **Transform** of a Scene Graph entity defines an entity's **translation**(location), **rotation**(orientation), and **scale**(size). To transform a Scene Graph entity, the entity must have a**transform_component**. The transform_component
  * **Origin:** An optional origin from which the LocalTransform is calculated.
  * **LocalTransform:** A` (/Verse.org/SpatialMath:)transform` relative to the entity's parent, or to Origin if the optional Origin field is set. 


The origin of an entity is the entity from which the LocalTransform of an entity's transform_component is calculated. This the value of the Origin field if the Origin field on the transform_componentis set, or the entity's parent in the Scene Graph hierarchy if the Origin field is not set. 
It is important to distinguish between a transform and the transform_component. A transform is a composite data type defined in the `/Verse.org/SpatialMath` module that stores the following quantities in the **Left-Up-Forward (LUF)** coordinate system:
  * **Translation**(`vector3`): The location of an object.
  * **Rotation**(`rotation`): The object's orientation.
  * **Scale**(`vector3`): The object's size.


The transform_component is a Verse class defined in the `/Verse.org/SceneGraph` module that stores a Scene Graph entity's LocalTransform and, optionally, an alternative Origin. You can think of the transform_component
###  Verse Transforms 
Scene Graph uses `/Verse.org/SpatialMath` module transforms. Verse module transforms use the right-handed LUF coordinate system. To construct a Verse transform, you can use the class archetype and built-in methods for creating `rotations`.
Verse 
```
# Transform specifying all fields
MyTransform:transform = transform:
	Translation := vector3{Left := 2.0, Up := -4.0, Forward := 8.0}
	Rotation := MakeRotationFromEulerDegrees(-90.0, 180.0, 0.0)
	Scale := vector3{Left := 2.0, Up := 4.0, Forward := 2.0}

# Transform specifying only Translation
MyOtherTransform:transform = transform:
	Translation := vector3{Up := 512.0}


```

Copy full snippet(15 lines long)
For more information about the coordinate system in UEFN, see the Left-Up-Forward Coordinate System page. This also includes information about converting between the XYZ transform in the `/UnrealEngine.com/Temporary/SpatialMath` module and the Verse module transform.
###  Construct an Entity with a transform_component 
You can construct an entity in the UEFN outliner or through Verse. 
###  In UEFN 
You can quickly add a Scene Graph entity to your project through the **Place Actors** menu.
[![](https://dev.epicgames.com/community/api/documentation/image/e3940c04-ef0f-4426-adb8-fe26144c2ba8?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/e3940c04-ef0f-4426-adb8-fe26144c2ba8?resizing_type=fit)
When you add a Scene Graph entity to your project through the **Place Actors** menu, the entity is equipped with a transform_component by default.
[![](https://dev.epicgames.com/community/api/documentation/image/0295b17b-48dc-489b-8727-e0649f8ae3cb?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/0295b17b-48dc-489b-8727-e0649f8ae3cb?resizing_type=fit)
If you want, you can remove the transform_component by navigating to your recently placed entity in the Outliner panel, finding the transform_component, and selecting **Remove Component**.
[![](https://dev.epicgames.com/community/api/documentation/image/d0e501d8-6991-4faf-8dae-4215dcdacb31?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/d0e501d8-6991-4faf-8dae-4215dcdacb31?resizing_type=fit)
You can see below that the entity in question no longer has a transform_component.
[![](https://dev.epicgames.com/community/api/documentation/image/73c5c26e-2339-489f-8693-3140672cf68a?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/73c5c26e-2339-489f-8693-3140672cf68a?resizing_type=fit)
###  In Verse 
When you create a Scene Graph entity through Verse code with the class archetype:
Verse 
```

```

MyEntity:entity = entity{}
Copy full snippet(1 line long)
The entity is not equipped with a transform_component. Instead, a transform_component must be added to your entity. The preferred methods for working with transforms are `SetLocalTransform` and `SetGlobalTransform`. These functions handle creating the transform_component, setting the values, and adding the transform component to the entity at the same time.
Verse 
```
MyEntity:entity = entity{}

# Add entity to simulation entity so MyEntity begins simulating when play begins
if (SimEntity := Entity.GetSimulationEntity[]):
	SimEntity.AddEntities(array{MyEntity})

# Identity transform
IdentityTransform:transform = transform:
	Translation := vector3{Left := 0.0, Up := 0.0, Forward := 0.0}
	Rotation := MakeRotationFromEulerDegrees(0.0, 0.0, 0.0)

```

Copy full snippet(19 lines long)
You can also add the component directly with `AddComponents`:
Verse 
```

```

# Add entity to simulation entity so MyEntity begins simulating when play begins if (SimEntity := Entity.GetSimulationEntity[]): SimEntity.AddEntities(array{MyEntity}) # Use AddComponents MyEntity.AddComponents(array{ transform_component{ Entity := MyEntity }}) # Same as above, but more readable if adding multiple components at once MyEntity.AddComponents of array: transform_component{Entity := MyEntity}
Copy full snippet(10 lines long)
Additional methods for adding a transform_component to a Scene Graph entity through Verse code are discussed in the Working with the transform_component in Verse section.
##  How an Entity's Transform is Calculated 
The most important operation you can perform on a Scene Graph entity with the transform_component is to get or set an entity's transform. 
There are two types of transform that you might be interested in: **local transform** and **global transform**. To effectively manipulate the transform of an entity, it is important to understand how these transforms are calculated. 
The **local transform** is the transform with respect to an entity's origin. The local transform of any Scene Graph entity is the value of the LocalTransform field on a Scene Graph entity's transform_component. 
The **global transform** is the transform with respect to the origin of the LUF coordinate system in global space. The global transform of a Scene Graph entity is calculated by composing an entity's LocalTransform with:
  * The entity's parent's global transform if the Origin field is not set on the entity's transform_component, or
  * The entity's origin's global transform if the Origin field is set on the entity's transform_component.


The following works through an example of this using translation with no rotation or scaling so that resulting transforms can be calculated by adding translations. Since this example assumes that the rotation and scale are left as their respective identities in all transforms, shorthand is used to refer to an entity's transform by the entity's transform translation vector.
Consider three Scene Graph entities organized as:
  * **BaseEntity:** LocalTransform (0.0, 0.0, 0.0) in the LUF coordinate system.
    * **A:** LocalTransform (750.0, 100.0, 0.0)
      * **B:** LocalTransform (-250.0, 100.0, 0.0)


Below are the three entities in a scene where **BaseEntity** has no mesh_component, **A** is a cube, and**B** is a cone:
[![](https://dev.epicgames.com/community/api/documentation/image/d8ed77d3-949e-4ff3-916d-c564e241ac43?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/d8ed77d3-949e-4ff3-916d-c564e241ac43?resizing_type=fit)
[![](https://dev.epicgames.com/community/api/documentation/image/748c195e-137d-4e8c-81b8-3ae840fed6cd?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/748c195e-137d-4e8c-81b8-3ae840fed6cd?resizing_type=fit)
For example, let's say you want to determine both the local transform and the global transform of entity **B**. The local transform is quick to see: it is the value of the LocalTransform field of the entity **B** transform_component as seen below:
[![](https://dev.epicgames.com/community/api/documentation/image/e4dff2cb-0953-4419-9d6d-8e820e645d20?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/e4dff2cb-0953-4419-9d6d-8e820e645d20?resizing_type=fit)
The local transform of entity **B** is (-250.0, 100.0, 0.0). 
In order to calculate the global transform of entity**B** , you would trace the hierarchy of entity **B** back to the simulation entity and compose transforms starting at the simulation entity, then down to entity **B**. The **Simulation Entity** is the root of the current simulation under which all other simulating entities are children. 
These are the steps to calculate the global transform of entity **B** :
  1. The simulation entity is the root of the current simulation.
     * Simulation Entity global transform: (0.0, 0.0, 0.0).
  2. The **BaseEntity** is a child of the simulation entity. The BaseEntity has a local transform of (0.0, 0.0, 0.0) as seen in Image 1.0. Composing the global transform of the Simulation Entity with the local transform of BaseEntity yields:
     * BaseEntity global transform: (0.0, 0.0, 0.0) + (0.0, 0.0, 0.0) = (0.0, 0.0, 0.0).
  3. Entity **A** is a child of BaseEntity. Entity **A** has a local transform of (750.0, 100.0, 0.0) as seen in Image 1.1. Composing the global transform of BaseEntity with the location transform of entity **A** yields:
     * Entity **A** global transform: (0.0, 0.0, 0.0) + (750.0, 100.0, 0.0) = (750.0, 100.0, 0.0).
  4. Entity B is a child of entity **A**. Entity **B** has a local transform of (-250.0, 100.0, 0.0) as seen in Image 1.2. Composing the global transform of **A** with the location transform of entity **B** yields:
     * Entity **B** global transform: (750.0, 100.0, 0.0) + (-250.0, 100.0, 0.0) = (500.0, 200.0, 0.0).


Therefore entity **B** 's global transform is (500.0, 200.0, 0.0) in the LUF coordinate system. 
##  Working with the transform_component in the Editor 
By using the LocalTransform and Origin you can manipulate an entity's transform. 
###  Local Transform 
The **LocalTransform** field of an entity's transform_component is the `(/Verse.org/SpatialMath:)transform` of an entity with respect to its Origin. If the Origin field is not explicitly set, the Origin of an entity defaults to its parent in the Scene Graph hierarchy. The LocalTransform field is an editable field that appears in UEFN where you directly edit an entity:
  1. **Translation:** Location relative to its parent or manually defined Origin.
     * Change the translation of an entity to move it around the level.
  2. **Rotation:** Orientation relative to its parent or manually defined Origin.
     * Change the rotation of an entity to rotate it around the entity's pivot point.
  3. **Scale:** Size relative to its parent or manually defined Origin.
     * Change the scale of an entity to make it larger or smaller.


###  Origin 
The **Origin** of an entity's transform_component is the object with respect to which the LocalTransform is computed. This field on the transform_component is optional. By default, the LocalTransform of the transform_component is the entity's transform based on the entity's origin, computed with respect to the owning entity's parent entity. You can optionally set the Origin field on an entity's transform_component to compute the LocalTransform with respect to an object other than the entity's parent. 
For example, you can set an entity's Origin to be a different entity entirely. You can set this field with any class that implements the Verse `origin` interface. In particular, to set the Origin field to another Scene Graph entity, choose the `entity_origin `class, then select the entity you want to be the new origin.
It is important to disambiguate between Origin with respect to the transform_component and the Origin of the LUF coordinate system. The Origin of the LUF coordinate system is the point in world space with Translation (0.0, 0.0, 0.0). 
The Origin of an entity's transform_component is the location at which an entity is located if the entity's transform_component LocalTransform has Translation (0.0, 0.0, 0.0). 
The following walks through an example of how the Origin field and LocalTransform work in practice. Consider a **BaseEntity** located at the origin of world space with child entity**A** represented by the cube mesh_component, and child entity **B** represented by the cone mesh_component. 
[![](https://dev.epicgames.com/community/api/documentation/image/ffe20027-f6d9-4fca-9881-1e357956708d?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/ffe20027-f6d9-4fca-9881-1e357956708d?resizing_type=fit)
As can be seen in the above image, **BaseEntity** is located at the origin of world space since its Origin field is unset in the transform_component and the LocalTransform is set to the Origin of world space. 
As shown below, you can see that the Origin field of entity **A** is also unset, so the LocalTransform is computed with respect to the transform_component of its parent, **BaseEntity**.
[![](https://dev.epicgames.com/community/api/documentation/image/b9194a54-a4da-4eca-8624-25c0f0f5ba33?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/b9194a54-a4da-4eca-8624-25c0f0f5ba33?resizing_type=fit)
Since **BaseEntity** is located at the origin of world space and the Translation of the LocalTransform of entity **A** is set to `vector3{Left := 750.0, Up := 100.0, Forward := 0.0}`, entity **A** is located at `vector3{Left := 750.0, Up := 100.0, Forward := 0.0}`. The Rotation and Scale of entity **A** remain unchanged since the **Rotation** and **Scale** of the LocalTransform on the **BaseEntity** are the respective identities.
In the image below, you can see that entity **B** , which is also a child of **BaseEntity** , has its Origin field on its transform_component set as entity **A**. 
[![](https://dev.epicgames.com/community/api/documentation/image/89811f2f-e1b9-486a-8e27-6cc6693e8ca4?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/89811f2f-e1b9-486a-8e27-6cc6693e8ca4?resizing_type=fit)
As a result, the entity **B** LocalTransform field on its transform_component is relative to the calculated transform of entity **A.** Since entity **B** 's LocalTransform is set to `vector3{Left := -250.0, Up := 100.0, Forward := 0.0}` and entity **A** 's LocalTransform is set to `vector3{Left := 750.0, Up := 100.0, Forward := 0.0}`, the global transform of entity **B** is `vector3{Left := 500.0, Up := 200.0, Forward := 0.0}`. 
If you change the Rotation component of the LocalTransform field of the entity A transform_component to -90.0 degrees about the Forward-axis, entity B also rotates -90.0 degrees about the Forward-axis. This is because the rotation of a child is also inherited from the parent.
[![](https://dev.epicgames.com/community/api/documentation/image/93ec813e-dec4-46be-b6d9-592817067474?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/93ec813e-dec4-46be-b6d9-592817067474?resizing_type=fit)
##  Working with the transform_component in Verse 
You can check whether an entity has a transform_component with `GetComponent`:
Verse 
```
MyEntityNoTransform:entity = entity{}

if (SimEntity := Entity.GetSimulationEntity[]):
	SimEntity.AddEntities(array{MyEntityNoTransform})

MyEntityWithTransform:entity = entity{}
MyEntityWithTransform.AddComponents of array:
    transform_component{Entity := MyEntityWithTransform}

if (SimEntity := Entity.GetSimulationEntity[]):

```

Copy full snippet(21 lines long)
Instead of retrieving the transform_component with `GetComponent `and working directly with the transform_component, the recommended way to interact with the transform_component is through the **entity extension methods**. 
###  Transform 
You can directly set the local or global transform of your entity with the **entity extension methods** `SetLocalTransform` or `SetGlobalTransform`.
Verse 
```
MyEntity:entity = entity{}
MyTransform:transform = transform{}

if (SimEntity := Entity.GetSimulationEntity[]):
	SimEntity.AddEntities(array{MyEntity})

# Set the local transform
MyEntity.SetLocalTransform(MyTransform)

MyOtherEntity:entity = entity{}

```

Copy full snippet(19 lines long)
These extension methods are the preferred way to set the transform on a Scene Graph entity in Verse code. These methods explicitly set the local (with respect to the entity's parent) or global (with respect to the world origin) transform of the entity in question. If the entity does not have a transform_component when either of these functions is called, a transform_component is implicitly created and added to the entity. After either of these function calls, the entity now has a `transform_component`. You can confirm this with a subsequent call to `GetComponent.`
Verse 
```

```

if (MyEntity.GetComponent[transform_component]): # success, entity has a transform_component else: # failure, entity does not have a transform_component 
Copy full snippet(4 lines long)
Using extension methods, you can obtain the local transform or global transform of an entity's associated transform_component. `GetLocalTransform` returns an entity's transform with respect to its parent entity or specified Origin field, if it is set. If the entity does not have a `transform_component`, this function returns the identity transform.
Verse 
```
# Entity object
MyEntity:entity = entity{}

if (SimEntity := Entity.GetSimulationEntity[]):
	SimEntity.AddEntities(array{MyEntity})

# Obtain the local transform with respect to Parent or Origin (if set)
EntityLocalTransform := MyEntity.GetLocalTransform() # no transform component, returns identity

MyOtherEntity:entity = entity{}

```

Copy full snippet(17 lines long)
`GetGlobalTransform` returns an entity's transform with respect to the world origin. If the entity does not have a `transform_component`, this function returns the global transform of the nearest ancestor with a `transform_component`.
Verse 
```

```

# Entity object MyEntity:entity if (SimEntity := Entity.GetSimulationEntity[]): SimEntity.AddEntities(array{MyEntity}) # Obtain the global transform EntityGlobalTransform := MyEntity.GetGlobalTransform() # returns global transform of Simulation Entity 
Copy full snippet(8 lines long)
###  Origin 
You can use entity extension methods for operations involving the transform_component Origin field as well. You can directly set the Origin field of your entity's transform_component with the entity extension method `SetOrigin`. 
Verse 
```
MyEntity:entity = entity{}
MyOtherEntity:entity = entity{}

if (SimEntity := Entity.GetSimulationEntity[]):
	SimEntity.AddEntities(array{MyEntity, MyOtherEntity})

MyEntity.SetLocalTransform(transform{Translation := vector3{Left := 100.0}})

# Construct an entity_origin object and set the new origin of MyEntity to MyOtherEntity
NewOrigin:entity_origin = entity_origin{Entity := MyOtherEntity}

```

Copy full snippet(15 lines long)
To obtain the Origin of an entity in Verse code, use `GetOrigin`.
Verse 
```
MyEntity:entity = entity{}

if (SimEntity := Entity.GetSimulationEntity[]):
	SimEntity.AddEntities(array{MyEntity})

if (OriginValue := MyEntity.GetOrigin[]):
	# should not succeed
else:
	# should fail, no transform_component on entity MyEntity


```

Copy full snippet(26 lines long)
`GetOrigin` decides whether an entity has its Origin field set and, if it does, returns the value of that field. If not, the call fails. Upon success, the return type of `GetOrigin` is an `origin` object, a Verse class that implements the `origin` interface. This interface provides a single function GetTransform to obtain the transform of the entity's specified origin. You can also check whether the alternative Origin field is set to an entity and determine which entity is set as your entity's Origin.
Verse 
```

```

if: OriginValue := MyOtherEntity.GetOrigin[] # get Origin object OriginEntityCast := entity_origin[OriginValue] # cast to entity_origin then: # Obtain the entity that is set as MyOtherEntity's Origin field on its transform_component MyOtherEntityOrigin := OriginEntityCast.Entity # Obtain the origin's transform TransformOfOrigin := OriginEntityCast.GetTransform() 
Copy full snippet(8 lines long)
Finally, you can reset the Origin field of an entity's transform_component with `ResetOrigin`.
Verse 
```

```

MyEntity.ResetOrigin()
Copy full snippet(1 line long)
After calling `ResetOrigin`, the Origin field of MyEntity's transform_component is reset and MyEntity's transform is now calculated with respect to MyEntity's parent entity in the Scene Graph hierarchy.
##  Dependent Components 
Some other Scene Graph components depend on the transform_component in order to function. These include:
  * light_component and its child classes
  * mesh_component
  * particle_system_component


If you add any of these components to a Scene Graph entity that does not already have a transform_component, the transform_component is added to the entity by default.
##  Note on XYZ Transforms 
Transforms using the XYZ coordinate system still exist in the `/UnrealEngine.com/Temporary/SpatialMath` Verse module. These transforms are still used by creative devices and props in the `/Fortnite.com` Verse module. For more information about the status of these transforms and converting between different transform types in Verse, see the [Left-Up-Forward Coordinate System](https://dev.epicgames.com/documentation/en-us/fortnite/leftupforward-coordinate-system-in-unreal-editor-for-fortnite) page.
  * [ gameplay](https://dev.epicgames.com/community/search?query=gameplay)
  * [ component](https://dev.epicgames.com/community/search?query=component)
  * [ scene graph](https://dev.epicgames.com/community/search?query=scene%20graph)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [ Verse Transforms ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#versetransforms)
  * [ Construct an Entity with a transform_component ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#constructanentitywithatransform-component)
  * [ In UEFN ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#inuefn)
  * [ In Verse ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#inverse)
  * [ How an Entity's Transform is Calculated ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#howanentity'stransformiscalculated)
  * [ Working with the transform_component in the Editor ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#workingwiththetransform-componentintheeditor)
  * [ Local Transform ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#localtransform)
  * [ Origin ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#origin)
  * [ Working with the transform_component in Verse ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#workingwiththetransform-componentinverse)
  * [ Transform ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#transform)
  * [ Origin ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#origin-2)
  * [ Dependent Components ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#dependentcomponents)
  * [ Note on XYZ Transforms ](https://dev.epicgames.com/documentation/en-us/fortnite/transforms-in-scene-graph-in-unreal-editor-for-fortnite#noteonxyztransforms)






---
