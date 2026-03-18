## https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite

# Progress Based Mesh Devices
Learn to use the Progress Based Mesh device to create a visual representation of progress.
![Progress Based Mesh Devices](https://dev.epicgames.com/community/api/documentation/image/76a57dd4-8723-4e59-b777-5681865091f6?resizing_type=fill&width=1920&height=335)
######  Prerequisite topics
In order to understand and use the content on this page, make sure you are familiar with the following topics:
  * [Getting Started with Devices](https://dev.epicgames.com/documentation/en-us/fortnite/getting-started-with-devices-in-fortnite)

The **Progress Based Mesh** device provides the option to create a visual system for the progress of an item. The device can swap between meshes and materials to visually represent different stages. The default mesh is a jar with a liquid material to show filling and draining.
You can use the device to simulate players placing objects inside other objects, track the progress of an event, and more. The device options and use cases change between Fortnite Creative and Unreal Editor for Fortnite (UEFN). To learn more, see the [Using the Device](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#using-the-device) section on this page.
##  Using the Device
The device creates a visual representation of progress. You can use buttons, triggers, and receivers for players to interact with the device.
If you're using multiple copies of a device on an island, you can rename them for organization. Choosing names that relate to a device's purpose helps to remember what each one does, and finding a specific device when using the [Event Browser](https://dev.epicgames.com/documentation/en-us/fortnite/event-browser-in-fortnite-creative).
The general flow of using the device is as follows:
  1. Place the device into your level.
  2. Set the[ progression values](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#user-options).
  3. Create a [threshold list of meshes](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#visuals-nbsp) (predefined in Creative).
  4. Trigger the [device's functions](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#direct-event-binding), or set the value directly in [Verse](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#verse-api) (UEFN only) to activate the threshold meshes.
  5. Add [visual and sound effects](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#additional-uefn-options-nbsp) (UEFN only).

For fundamentals on how to find, place, and customize a device, see Getting Started with Devices.
###  Creative
In Creative, the Progress Based Mesh devices come with a predefined list of threshold meshes that are not configurable.
You can adjust the options around the progression values, functions, and events. The material for the device is dynamic, meaning you can rotate the jar, and the liquid physically moves with it.
###  UEFN
In UEFN, you can use the default or custom meshes to create a mesh sequence. The default jar behaves the same as in Creative.
You can't change the static mesh from the component. You must use the [Threshold Mesh](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#visuals-nbsp) option to add meshes. When the progress of the device changes, the static mesh component updates with the active threshold mesh.
You can build a range of mechanics like:
  * Growing or decaying plants in a garden
  * The filling and draining of fuel tanks
  * Progress bar for players' rank in a game
  * Tip jar for your restaurant tycoon

The device also writes its progression state to the mesh's material via a **FillAmount** scalar material parameter. You can create your own materials using this parameter to get smooth transitions. This parameter becomes active through the **Fill Material Index** in the [Visuals](https://dev.epicgames.com/documentation/en-us/fortnite/using-progress-based-mesh-devices-in-fortnite#visuals-nbsp) category of the device.
The index represents the material slot attached to your static mesh. For the fundamentals of working with materials, see [Materials in UEFN](https://dev.epicgames.com/documentation/en-us/fortnite/materials-in-unreal-editor-for-fortnite).
To assign the material index:
  1. In a new or existing material, create a **ScalarParameter** node.
  2. Set the **Parameter Name** to **FillAmount**. You must use this name for the device to register the **Fill Material Index**.
  3. Connect the node as needed in the material graph.
  4. Assign the material to your static mesh.
  5. In the **Threshold Mesh** list, set the **Fill Material Index** to the material slot containing the **FillAmount** parameter. Only one material slot per mesh supports the fill parameter.

To view and adjust the material slots, open the mesh in the Static Mesh Editor and use the Details panel.
[![](https://dev.epicgames.com/community/api/documentation/image/20c5d622-158c-4550-8266-3a0deed7ff59?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/20c5d622-158c-4550-8266-3a0deed7ff59?resizing_type=fit) Progress Bar Material
##  Contextual Filtering
Some devices are affected by a feature called contextual filtering. This feature hides or displays options depending on the values selected for certain related options. This organization reduces clutter in the Details panel, helping to manage and navigate the settings. To identify these options, values that trigger contextual filtering in the settings tables on this page are in italic.
All options are listed in the following sections, including those affected by contextual filtering. If an option is hidden or displayed based on a specific value, there will be a note about it in the **Description** field of the table for that option.
##  Device Options
The core options for the device are the target value for complete progress and the rate of progression.
You can configure this device with the following options. Default values are bold. Values that trigger contextual filtering are italic.
|  |
---|---|---
Option |  Value |  Description
**Progress Target** |  **100** , choose a value |  The maximum progress for the device to reach. Value ranges from 0 - 100.
**Game Start Progress Amount** |  **0** , choose a value |  The amount of progress the device has at the start of the game. Value ranges from 0 - 100.
**Progress Rate** |  **5** , choose a value |  The rate at which to increase the progress amount based on the **Progression Type**.  If set to continuous, it's the rate at which progress changes. If set to instant rate, it's how much to change by per event call. Value ranges from 0 - 100.
**Regress Rate** |  **5** , choose a value |  The rate at which to decrease the progress amount based on the **Progression Type**.  If set to continuous, it's the rate at which progress changes. If set to instant rate, it's how much to change by per event call.  Value ranges from 0 - 100.
**Progression Type** |  **Continuous Rate** , Instant |  Options for how the progress amount updates.
  * **Instant Rate:** Gains a single chunk of the Progress or Regress Rate at once.
  * **Continuous Rate:** Updates at the specific Progress or Regress Rate per second.

##  Functions and Events
For more information on how events and functions work, see [Getting Started with Devices](https://dev.epicgames.com/documentation/en-us/fortnite/getting-started-with-devices-in-fortnite).
###  Functions
Functions  |  Description
---|---
**Begin Progressing When Receiving From** |  Increases the current progress level by the **Progress Rate** user option.
**Begin Regressing When Receiving From** |  Reduces the current progress level by the **Regress Rate** user option.
**Pause When Receiving From** |  Pauses the device.
**Progress Fully** |  Increases the device to its **Progress Target**.
**Regress Fully** |  Reduces the **Progress Target** to 0.
###  Events
Events in UEFN are read-only. When you set a function on another device that binds to an event on this device, the events are set automatically but cannot be edited.
In Creative, you can link events to functions as well as functions to events.
Events  |  Description
---|---
**On Progress Filled Completely** |  Event that occurs when the device reaches its **Progress Target**.
O**n Progress Emptied Completely** |  Event that occurs when the device regresses to 0.
**On Progress Changed** |  Event that occurs when the current progress of the device changes.
**Progress Threshold Cross Event** |  Event that occurs when the device hits one of the mesh thresholds, and a mesh is swapped in response.
##  UEFN-Only Options
###  Visuals
Use the **Visuals** category to adjust the appearance of the mesh and materials at different thresholds. The default value is the jar mesh at different fill stages.
Visual Options  |  Value  |  Description
---|---|---
**Threshold Mesh** |  _Index_ |  Represents the list of meshes for the stages of progression. To add meshes to the list, click the plus (**+**) icon.
**Threshold** |  Minimum (Min) Maximum (Max) |  Sets the progress range (the bound) for the mesh to be active.  Use the following options to determine how the set min and max values are included in the range.
  * **Exclusive:** Excludes the set value.
  * **Inclusive (Default):** Includes the set value.
  * **Open:** Uses the whole range, from the set value to the **Progress Target**.

If two thresholds overlap, the device uses the first qualifying threshold in the list.
**Static Mesh** |  Choose a Static Mesh Asset |  Sets the mesh for the threshold range. The mesh that the device will show while its progress value falls between that threshold.
**Transition VFX** |  Choose a Niagara System |  Simulates the visual effect (VFX) when the device transitions into the set static mesh.
**Transition Sound Cue** |  Choose a Sound Cue Asset |  Plays the sound when the device transitions into the set static mesh.
**Fill Material Index** |  **2** , choose a number |  Creates a dynamic material instance for the material in this slot, and writes to the **FillAmount** scalar material parameter.  This material parameter for the current fill is expressed as a ratio of `Current Progress / Progress Target`. For example, if your target progress is 100 vs 50, but you have a current progress of 25, then you'll get 1/4 full vs 1/2 full. To disable the functionality, set the value to -1.  You must use a **ScalarParameter** node in the material, and rename it to **FillAmount**.
###  Audio
With **Continuous Rate** active, you can add audio to indicate the progress.
Audio Type  |  Description
---|---
**Progress Audio** |  Plays audio when the device is progressing at a continuous rate.
**Regress Audio** |  Plays audio when the device is regressing at a continuous rate.
**Finish Audio** |  Plays audio when the device reaches its **Progress Target**.
The following general categories are included in the Details panel:
  * HLOD
  * Displacement
  * Rendering
  * Draw Distance
  * Data Layers

To learn more about the panel, see [User Interface Reference](https://dev.epicgames.com/documentation/en-us/fortnite/user-interface-reference-for-unreal-editor-for-fortnite).
##  Verse API
You can use the Verse API for the Progress Based Mesh device to customize your further mechanics. In Verse, you can directly set the progress amount. When coupled with triggers and receivers, you can configure pre-determined progress and regression amounts.
For more information on using the device in Verse, see the `progress_based_mesh_device` API reference.
