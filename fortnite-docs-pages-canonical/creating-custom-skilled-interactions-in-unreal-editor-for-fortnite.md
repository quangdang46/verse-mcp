## https://dev.epicgames.com/documentation/en-us/fortnite/creating-custom-skilled-interactions-in-unreal-editor-for-fortnite

# Creating Custom Skilled Interactions
Use Viewbindings to connect your custom UI to the Skilled Interaction device.
![Creating Custom Skilled Interactions](https://dev.epicgames.com/community/api/documentation/image/49dc0fb5-ab24-497e-af33-8d4001d712e0?resizing_type=fill&width=1920&height=335)
This walkthrough provides an example of a **UMG** (Unreal Motion Graphics) design and its **View Model** bindings that you can use to create a custom UI for the [Skilled Interaction device](https://dev.epicgames.com/documentation/en-us/fortnite/using-skilled-interaction-devices-in-fortnite-creative).
As you create your custom skilled interaction, make sure to set an event to begin the interaction. You can also set event triggers that grant players items for successfully completing the skilled interaction.
You can build on these examples by setting [cinematic cutscenes](https://dev.epicgames.com/documentation/en-us/fortnite/making-cinematics-and-cutscenes-in-unreal-editor-for-fortnite) once players or objects target certain zones.
[![Golf Example](https://dev.epicgames.com/community/api/documentation/image/998f476b-2886-4d72-97dd-6a835eb35d23?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/998f476b-2886-4d72-97dd-6a835eb35d23?resizing_type=fit)
This tutorial covers how to create a quick-press skilled interaction in which players can hold a trigger input to target the correct zones.
Before you begin customizing your UI, must create and import any assets to use in your designs. Visit [](https://dev.epicgames.com/documentation/en-us/fortnite/creating-custom-ui-from-materials-in-unreal-editor-for-fortnite)**[Creating Custom UI with Material Instances](https://dev.epicgames.com/documentation/en-us/fortnite/creating-custom-ui-with-material-instances-in-unreal-editor-for-fortnite)** to learn more about using materials in your design.
##  Set up the Device
Follow the steps below to create a golf example in which players target a perfect zone to grant success when hit. When designing your UI, feel free to rename the panels as you place them in the **Hierarchy** panel.
  1. From the **Content Browser** , place a Skilled Interaction device into your project.
  2. In the **Details** panel for the device, modify the following settings.
[![Modified Golf Example](https://dev.epicgames.com/community/api/documentation/image/79dd312f-11d5-4c55-a9dd-40e978757eb1?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/79dd312f-11d5-4c55-a9dd-40e978757eb1?resizing_type=fit)
Option  |  Value  |  Description
---|---|---
**UI Type** |  Bar |  Determines the type of user interface to display.
**Custom Widget** |  Add your custom **User Widget** |  Select a custom widget to use for the interaction.
**Interaction Type** |  Charge and release |  Charge and Release animates while holding the trigger button and activates upon release.
**Meter Color** |  Pick a color |  This example uses the color red.
**Good Zone Size** |  80.0 |  Sets the good zone size as a percent of the total meter.
**Good Zone Position** |  0.0 |  Sets the good zone's position.
**Perfect Zone Size** |  15.0 |  Sets the size of the perfect zone as a percent of the good zone.
**Perfect Zone Position** |  100.0 |  Sets the position of the perfect zone.
**Good Zone Color** |  Pick a color |  This example uses the color dark green.
**Perfect Zone Color** |  Pick a color |  This example uses the color lime green.

##  Add the Background Image
[![Golf Example](https://dev.epicgames.com/community/api/documentation/image/b430c439-7292-4d95-b3c7-2c6c4f1f032c?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/b430c439-7292-4d95-b3c7-2c6c4f1f032c?resizing_type=fit)
Follow the steps below to create the background image for the vertical meter bar in this example. The background in this walkthrough is a rounded, black bar in which the zones will sit on top of.
[![](https://dev.epicgames.com/community/api/documentation/image/9fe424a6-e767-4914-a605-fde4cae1a5c4?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/9fe424a6-e767-4914-a605-fde4cae1a5c4?resizing_type=fit)
Use the image above as a reference when recreating the steps in this tutorial.
  1. Create a **User Widget** for your interaction.
  2. Locate and double-click the **User Widget** attached to the device to access its **User Widget Editor**.
  3. In the **Hierarchy** panel, drag and nest an **Overlay** to serve as the overall canvas. In this example, it is named Overlay. Then, drag another **Overlay** to contain the background. In this example, it is named SID.
  4. Drag and nest an **Image** , named Bar in the example, into the child **Overlay**.
  5. In the **Details** panel of the **Image** , set the material or texture for your background.

##  Set up the Zones
You can create a bad zone for your example that will grant an automatic failure when targeted. However, this example only uses a perfect zone, which consists of three containers:
  * An initial empty area, called **PerfectZoneStart**.
  * The good or perfect zone, called **SizeBox**.
  * The empty area after, called **PerfectZoneEnd**.

Follow the steps below to recreate this example's perfect zone.
[![Golf Example](https://dev.epicgames.com/community/api/documentation/image/08bd3fb1-2035-48dd-b829-56a43a78a0c8?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/08bd3fb1-2035-48dd-b829-56a43a78a0c8?resizing_type=fit)
  1. From the **Palette** panel, drag and nest a **Stack Box** inside the child **Overlay** , named SID in the example.
  2. Then, drag and nest the following children inside the **Stack Box** : **Scale Box** > **Size Box**.
    1. In the **Details** panel for the **Scale Box** panel, set the **Stretch** option to **User Specified**. Then, set the **User Specified Scale** to indicate how much space you want above the perfect zone. For the purpose of this example, set the scale to **0.27**.
    2. In the **Size Box** , set the **Height Override** setting to the height of the top meter.
  3. Inside the same **Stack Box** , also drag a **Size Box** > **Scale Box** > **Overlay** > **Image**.
    1. In the **Details** panel for the **Size Box** , set the **Height Override** to the size of the perfect zone.
    2. In the **Details** panel for the **Scale Box** , set the **Stretch** setting to **User Specified**. Then, set the **User Specified Scale** setting to **1**.
    3. The **Overlay** PerfectZone will contain the material or texture for the perfect zone.
  4. Inside the same **Stack Box** , also drag an **Overlay** > **Size Box**.
    1. In the **Details** panel for the **Overlay** PerfectZoneEnd, set the **Stretch** setting to **User Specified**. Then, set the **User Specified Scale** to indicate how much space you want above the perfect zone. For the purpose of this example, set the scale to **0.49**.
[![](https://dev.epicgames.com/community/api/documentation/image/639d58b3-032e-4d42-a390-18d402e91c9c?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/639d58b3-032e-4d42-a390-18d402e91c9c?resizing_type=fit)
    1. In the **Details** panel for the **Size Box** , set the **Height Override** setting to the height of the bottom meter.
[![](https://dev.epicgames.com/community/api/documentation/image/9d3be976-ae1d-447e-ac3a-c8ab5f2f261f?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/9d3be976-ae1d-447e-ac3a-c8ab5f2f261f?resizing_type=fit)
  5. To add extra detail, you can include notches for the background zones by adding an **Image** , named Notches in the example, underneath the **Stack Box**.

##  Set up the Scrubber
For this example, the scrubber needs to be inside a moving container. To do this, you must create a **Stack Box** with two items.
The first item holds a **Size Box** set to **User Scale** , which you are later going to bind to the **Skilled Interaction Meter Scale** in the **ViewModel**. The second item will hold a container with the scrubber.
[![Scrubbers](https://dev.epicgames.com/community/api/documentation/image/0e76316b-ea41-4bbd-970e-ea78e39d93ac?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/0e76316b-ea41-4bbd-970e-ea78e39d93ac?resizing_type=fit)
  1. Underneath the **Overlay** SID drag and nest a **Stack Box** , named ScrubberStackBox in the example, to contain two child setups of: **Overlay** > **Size Box** and **Size Box** > **Scale Box** > **Image** named Scrubber.
    1. In the **Details** panel for the **Overlay** ScrubberMovingZone, set the **Stretch** setting to **User Specified**. Then, set the **User Specified Scale** setting to **0**.
[![](https://dev.epicgames.com/community/api/documentation/image/ef6db1ac-2c5d-4c1f-83cf-967b87f28aa0?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/ef6db1ac-2c5d-4c1f-83cf-967b87f28aa0?resizing_type=fit)
    1. In the **Details** panel for the **Size Box** , set the **Height Override** setting to the height of the top meter.
[![](https://dev.epicgames.com/community/api/documentation/image/8610cdad-8ba0-49ad-9bbd-b9c6d65ac86d?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/8610cdad-8ba0-49ad-9bbd-b9c6d65ac86d?resizing_type=fit)
    1. Set the **Height Override** setting for **Size Box** ScrubberContainer to **0**.
[![](https://dev.epicgames.com/community/api/documentation/image/1fbe900e-466c-4837-9aa2-4ab20d387d38?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/1fbe900e-466c-4837-9aa2-4ab20d387d38?resizing_type=fit)
    1. Change the **Stretch** setting for the **Overlay** Scale Box to **User Specified**. Set the **User Specified Scale** setting to **1**.
[![](https://dev.epicgames.com/community/api/documentation/image/93d6f514-eff2-4cf8-9c92-b658c16402e8?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/93d6f514-eff2-4cf8-9c92-b658c16402e8?resizing_type=fit)
    1. In the **Image** setting for the **Image** Scrubber, set the material or texture of your scrubber.
[![](https://dev.epicgames.com/community/api/documentation/image/8e80a3c3-c695-4eb2-b44c-3c8533637c12?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/8e80a3c3-c695-4eb2-b44c-3c8533637c12?resizing_type=fit)

##  Set up the ViewModel
To connect your custom UI to the Skilled Interaction device, follow these steps.
  1. In the **User Widget** , navigate to **Window** > **Viewmodels** to open the **Viewmodels** window.
  2. Click **+Viewmodel**. Then, select **Device - Skilled Interaction View Model** and click **Select**.
  3. Either from the bottom toolbar or the **Window** tab, select **View Bindings**.
  4. Set up your **View Bindings** to match the image below.
[![Viewbindings](https://dev.epicgames.com/community/api/documentation/image/f63067cd-dd3b-4818-acc5-a83bdbb46605?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f63067cd-dd3b-4818-acc5-a83bdbb46605?resizing_type=fit)
    1. Click **+ Add Widget** to add the **ScrubberMovingZone**.
    2. Set the **ScubberMovingZone** to **User Specified Scale** and **UEFN_SkilledInteraction_ViewModel** to **Current Meter Value**.
    3. Click **+ Add Widget** to add the **PerfectZoneStart**.
    4. Set the **PerfectZoneStart** to **User Specified Scale** and **UEFN_SkilledInteraction_ViewModel** to **Perfect Zone Min**.
    5. Click **+ Add Widget** to add the **PerfectZoneEnd**.
    6. Set the **PerfectZoneStart** to **User Specified Scale** and add a conversion function for **Add Int Double**.
    7. Set **A** to **1**.
    8. Set **B** to **UEFN_SkilledInteraction_Viewmodel/Perfect Zone Max**.
    9. Set **Negate B** to **True**.
