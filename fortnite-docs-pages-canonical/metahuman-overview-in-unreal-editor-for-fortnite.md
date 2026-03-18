## https://dev.epicgames.com/documentation/en-us/fortnite/metahuman-overview-in-unreal-editor-for-fortnite

# MetaHuman Overview
MetaHuman is available to use in Unreal Editor for Fortnite! Discover a new world of possibilities.
![MetaHuman Overview](https://dev.epicgames.com/community/api/documentation/image/39a6a795-361b-4f92-ad27-7fca5d30bfbf?resizing_type=fill&width=1920&height=335)
**MetaHuman** , the state-of-the-art system allowing you to create high-fidelity digital humans, is now available and free to use in Unreal Editor for Fortnite!
Below you will find an overview of the features available in UEFN and links to the Unreal Engine [MetaHuman documentation](https://dev.epicgames.com/documentation/metahuman/metahuman-documentation) for in-depth information.
##  Creating your MetaHuman
The MetaHuman Creator is fully integrated into [Unreal Engine](https://www.unrealengine.com/download), providing you with greater flexibility and customization in creating and adding MetaHumans directly into your project. Here's a quick overview of the steps to
  1. Prepare Unreal Engine for the [MetaHuman Creator](https://dev.epicgames.com/documentation/en-us/metahuman/metahuman-creator?application_version=5.6).
  2. [Create your MetaHuman](https://dev.epicgames.com/documentation/en-us/metahuman/creating-a-character?application_version=5.6).
  3. [Export your creation](https://dev.epicgames.com/documentation/en-us/metahuman/assembly?application_version=5.6).
To export to UEFN, choose **UEFN Export** for the **Assembly** field. Make sure that UEFN is closed before assembling a character.
[![MetaHuman Export](https://dev.epicgames.com/community/api/documentation/image/1a91e0fc-01eb-48e3-b6f8-971fa4a5f98c?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/1a91e0fc-01eb-48e3-b6f8-971fa4a5f98c?resizing_type=fit) MetaHuman Export

###  Legacy Cloud Workflow
If you want to use [MetaHuman Creator in the cloud](https://dev.epicgames.com/documentation/en-us/metahuman/metahuman-creator-overview?application_version=5.0-5.5), you can follow the steps outlined in the older version of the MetaHuman documentation. Be advised that no new updates will be made to the cloud version. For the latest features, use the new [MetaHuman Creator for Unreal Engine](https://dev.epicgames.com/documentation/en-us/metahuman/metahuman-creator).
The earlier version of [**MetaHuman Creator**](https://metahuman.unrealengine.com/) is a cloud-based tool that allows you to create your own digital human, starting from one of the MetaHuman preset characters.
[![MH Creator](https://dev.epicgames.com/community/api/documentation/image/82b86601-9d92-44d9-b819-41dfdfa66366?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/82b86601-9d92-44d9-b819-41dfdfa66366?resizing_type=fit)
First, select a starting point from the presets. Next, choose additional presets to blend into your MetaHuman. Finally, refine your character with easy sculpting tools and control guides.
For more complete information on this method of creating your MetaHuman, see the [MetaHuman Creator section](https://dev.epicgames.com/documentation/en-us/metahuman/metahuman-creator-overview?application_version=5.0-5.5) of the documentation.
##  Importing your MetaHuman into UEFN
[![MH Importer window](https://dev.epicgames.com/community/api/documentation/image/cf3b3b2b-b411-4a57-8340-247b5e3bbcea?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/cf3b3b2b-b411-4a57-8340-247b5e3bbcea?resizing_type=fit)
Once you have created your MetaHuman, open your project in **UEFN**.
From the **Window** dropdown menu, select **MetaHuman Importer**. You can also right-click in the **Content Browser** and select the importer from the dropdown menu.
[![MH Importer](https://dev.epicgames.com/community/api/documentation/image/4f9c908d-df60-4f7f-bf80-8886802e96ac?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4f9c908d-df60-4f7f-bf80-8886802e96ac?resizing_type=fit)
For the complete process, see [Setting Up Your MetaHuman in UEFN](https://dev.epicgames.com/documentation/en-us/metahuman/metahuman-in-unreal-editor-for-fortnite).
##  Animating Your MetaHuman
Once your MetaHuman is imported to UEFN, there are four ways to go about animating it:
  * Play a pre-made custom animation
  * Animate directly using Sequencer
  * Use the NPC Spawner
  * Use Performance Capture

Make sure your imported MetaHuman is already in your scene before starting.
###  Playing a Pre-made Custom Animation
The quickest way to animate a MetaHuman is to apply a ready-made animation to the MH mesh. MetaHumans come with a preloaded set of animations that can be found in the **MetaHumans** > **Common** > **Common** > **Locomotion** folder. If you want to import a custom animation, follow the steps below:
  1. Import an FBX animation of your choice to use with your MetaHuman.
    1. Create a new folder called **MyAnimations** inside your MetaHumans folder.
    2. Right-click inside the Content Browser and select the **Import** function, or drag and drop the animation file into the newly-created folder.
    3. On the **FBX Import Options** popup, select the **metahuman_base_skel** from the **Skeleton** dropdown menu, then click **Import All**.
[![select skeleton](https://dev.epicgames.com/community/api/documentation/image/290a9ac9-abce-4adf-bae1-1b87a95c1e34?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/290a9ac9-abce-4adf-bae1-1b87a95c1e34?resizing_type=fit)
  2. Select a MetaHuman from your scene, click on the **Body** component, then drag the animation into the **Anim to Play** field.
  3. To preview and edit the animation itself, double-click on the animation file to open a new editor window.
[![Animation editor](https://dev.epicgames.com/community/api/documentation/image/f1705d52-6e6c-4016-bc5e-812e78faf634?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f1705d52-6e6c-4016-bc5e-812e78faf634?resizing_type=fit)
    1. Click on **Character** > **Bones** >**All Hierarchy** to see the skeleton moving with the animation.
    2. You can add preview meshes to the animation. Add **Array Elements** by clicking the **+** next to the **Skeletal Meshes** field and selecting a mesh for the top **Skeletal Mesh** element.
[![preview meshes](https://dev.epicgames.com/community/api/documentation/image/58380659-54e5-4e44-b70e-0e39eba12bde?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/58380659-54e5-4e44-b70e-0e39eba12bde?resizing_type=fit)
Preview meshes have been removed due to large asset size. Adding them will incur a cost to your project size. The cost is added if you save the animation file with the additional skeletal meshes added.

###  Animating in Sequencer
Start this process by creating a **Level Sequence** in your Content Browser. Right-click inside the browser, then select **Cinematics** > **Level Sequence**.
Check out the detailed process for animating MetaHumans on the [Using MetaHumans in Sequencer](https://dev.epicgames.com/documentation/en-us/metahuman/animate-using-sequencer) page.
To get the animation to play in your UEFN experience, you must link the created Level Sequence to a [Cinematic Sequence device](https://dev.epicgames.com/documentation/uefn/cinematic-sequence-device-in-unreal-editor-for-fortnite). For more information on this process, see [Sequencer and Control Rig in UEFN](https://dev.epicgames.com/documentation/uefn/sequencer-and-control-rig-in-unreal-editor-for-fortnite).
###  Using the NPC Spawner
The [NPC Spawner](https://dev.epicgames.com/documentation/uefn/using-npc-spawner-devices-in-unreal-editor-for-fortnite) is a powerful tool designed to bring Fortnite and other custom non-player characters (NPCs) into your games.
Create a character definition with the settings below. To learn more about the process of adding NPC character definitions to a mesh, see the [NPC Character Definitions](https://dev.epicgames.com/documentation/uefn/npc-character-definitions-in-unreal-editor-for-fortnite) page.
[![NPC Character definition](https://dev.epicgames.com/community/api/documentation/image/4d84cefc-56f5-4bfc-941e-8fe6374d1f37?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4d84cefc-56f5-4bfc-941e-8fe6374d1f37?resizing_type=fit)
Option  |  Value  |  Description
---|---|---
**Type** |  Custom |  Behaviors will be defined in Verse.
**Behavior** |  Verse Behavior |  Available with all character types. Allows you to define the behavior for your character.
**Cosmetic Modifier** |  |
**Character Look** |  Custom Character |  This is equivalent to a non-Fortnite character.
**Character Blueprint** |  Your MetaHuman BP |  Choose your MetaHuman.
**Character Movement** |  Animation Preset |  Allows you to choose an animation preset to add to your MetaHuman
**Anim Preset** |  AnimPreset_MetaHumanLocomotion |  This is the animation preset that retargets onto the MetaHuman skeletal mesh and applies default MetaHuman locomotion animation.
To give your MetaHuman custom behavior, check out [Creating Custom NPC Behavior](https://dev.epicgames.com/documentation/uefn/create-custom-npc-behavior-in-unreal-editor-for-fortnite) using Verse.
###  Using Performance Capture
MetaHuman Animator (MHA) feature allows you to quickly and easily capture footage of body and face motions, then apply them to your MetaHuman inside UEFN.
[![Facial Animation](https://dev.epicgames.com/community/api/documentation/image/4107d7cb-f65a-4dee-af1c-da67335d22c7?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4107d7cb-f65a-4dee-af1c-da67335d22c7?resizing_type=fit)
You can access these tools by right-clicking in your UEFN Content Browser and selecting **MetaHuman Animator**.
[![Accessing MetaHuman animator](https://dev.epicgames.com/community/api/documentation/image/67da805f-81fe-43ab-90fe-7dfeeb0f71c1?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/67da805f-81fe-43ab-90fe-7dfeeb0f71c1?resizing_type=fit) Accessing MetaHuman animator
For a walkthrough of the LiveLink Hub, see [Using LiveLink Hub in UEFN](https://dev.epicgames.com/documentation/en-us/fortnite/using-livelink-hub-in-unreal-editor-for-fortnite).
###  Current Limitations for MetaHuman Animator
Feature  |  UEFN Impact
---|---
Level sequence export requires the use of various features including Spawnables, which are not supported in UEFN. |  Level sequence export is not supported.
The maximum length of sound assets supported in UEFN is 300 seconds, soon to be 900 seconds. |  Attempting to ingest footage longer than 15 minutes will give a warning and not allow it.
The CameraCalibration plugin is not supported in UEFN. |  You will see the default asset editor for LensFile assets.
Media cache settings are not editable. |  Tend to get skipping/issues when viewing depth. We reduced impacts by making depth track auto-mute when not visible.
Capture manager save location |  Capture Manager will restrict where you can save data to try and be consistent with UEFN.
Template to MetaHuman |  Template to MetaHuman will not be available in UEFN. Users could do the template in UE, push to MHC, then import into UEFN via importer.
Applying an MHA animation sequence on a Fortnite character. |  This will reset animation on the entire skeleton. Any existing body or facial animation will be overwritten.
Custom tracking models |  You aren't able to import custom tracking models to UEFN.
##  Clothing For Your MetaHuman
Marvelous Designer, a state-of-the-art digital clothing software, has partnered with Epic Games to let UEFN developers make clothing for their MetaHumans using their software.
The general workflow to bring clothes for MetaHumans into UEFN has three steps.
[![Clothing pipeline](https://dev.epicgames.com/community/api/documentation/image/4321d17c-a55a-4af4-bd3b-33a430f0c301?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4321d17c-a55a-4af4-bd3b-33a430f0c301?resizing_type=fit)
The [Creating Clothing for UEFN using Unreal Engine](https://dev.epicgames.com/documentation/en-us/fortnite/creating-clothing-assets-for-unreal-editor-for-fortnite-using-unreal-engine) pages cover the process illustrated above.
###  Create the Clothing Assets in Marvelous Designer
Learn more about [Marvelous Designer](https://www.marvelousdesigner.com/). The following tutorials can help you get familiar with Marvelous Designer’s workflows:
  * [Marvelous Designer 11 Tutorial: Workflow for Games](https://www.youtube.com/watch?v=2WlVX4dQrJc)
  * [Marvelous Designer 12.1 USD compatibility & Omniverse Connector](https://www.youtube.com/watch?v=Hto1RKwyzwk)
  * [Marvelous Designer Workflow: Marvelous Designer Omniverse Connector](https://www.youtube.com/watch?v=r7AfPVd8WG4)

To celebrate this partnership, UEFN developers can take advantage of free one-year Marvelous Designer licenses. For more information on the partnership, see the [Talisman Demo Templates](https://create.fortnite.com/news/new-talisman-demo-templates-now-available-for-uefn) announcement.
###  Import Assets into Unreal Engine to Create the Clothing Asset
Learn more about creating clothing assets in Unreal Engine using the [Cloth Panel Editor](https://dev.epicgames.com/community/learning/tutorials/pv7x/unreal-engine-cloth-panel-editor).
###  Migrate the Clothing Asset into UEFN
Once your clothing asset is ready in Unreal Engine, use the **Migrate Tool** to bring it to UEFN.
  1. Right-click on your ClothAsset file and select **Asset Actions** > **Migrate**.
[![Migrate tool menu](https://dev.epicgames.com/community/api/documentation/image/c23a3a96-19e8-4a76-82c8-d2dac5adea42?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/c23a3a96-19e8-4a76-82c8-d2dac5adea42?resizing_type=fit)
  2. Choose the files you want to migrate and click **OK**.
[![Asset report](https://dev.epicgames.com/community/api/documentation/image/debaa8ec-f5f4-4ca5-b7db-d35190df83c9?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/debaa8ec-f5f4-4ca5-b7db-d35190df83c9?resizing_type=fit)
  3. Select the **Your_UEFN_Project** > **Common** folder as the destination and migrate the files.
  4. Open your UEFN project and you should see the ClothAsset under your project files.

##  Unsupported in UEFN
The following MetaHuman features are currently not included in UEFN:
**Physics Materials**
These are specialized materials that determine how a physics object will react when it touches the material in question. It has no incidence on how objects look, but determine characteristics such as friction on a surface. See the [Physics Materials](https://docs.unrealengine.com/physical-materials-in-unreal-engine/) page in Unreal Engine to find out more.
**Pose Assets**
Pose assets are created from an animation asset and represent a single animation frame for a specific skeletal mesh. These are usually used as references. Read the [Animation Pose Assets](https://docs.unrealengine.com/animation-pose-assets-in-unreal-engine/) page to learn more.
**Editor Utility Widget (Control Rig Picker)**
Animating hands and other body parts in UEFN may be a bit trickier, Editor Utility Widgets can add logic via Blueprints and are useful for artists for automating a lot of tedious work. See [Editor Utility Widgets](https://docs.unrealengine.com/editor-utility-widgets-in-unreal-engine/) in Unreal Engine for more info.
